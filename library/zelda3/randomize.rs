use assembly::zelda3::get_patch_data;
use common::string_to_hash;
use common::RomSize;
use common::SnesGame;
use log;

use super::io::write_model;
use crate::zelda3::features::apply_features;
use crate::zelda3::io::read_model;
use crate::zelda3::model::Z3Model;
use crate::zelda3::options::Z3Options;

pub fn randomize_zelda3(bytes: &[u8], options: &Z3Options) -> Vec<u8> {
    let mut game: SnesGame = create_game(bytes);
    let mut model: Z3Model = read_model(&game);

    // Copy options onto the model.
    model.debug_string = options.seed.to_string();
    model.seed = string_to_hash(options.seed.as_str());
    model.uw_balancing = options.underworld_balancing;
    model.ow_balancing = options.overworld_balancing;
    apply_features(&mut model, options);
    write_model(&mut game, &model);

    game.write_crc();
    game.buffer
}

fn create_game(bytes: &[u8]) -> SnesGame {
    log::info!("Original Size={}", bytes.len());

    // Unconditionally strip SMC headers.
    let bytes: Vec<u8> = match bytes.len() % 0x400 == 0x200 {
        true => {
            log::info!("Removing unnecessary header");
            bytes[0x200..].to_vec()
        }
        false => bytes.to_vec(),
    };

    // Resize to 4MB to make additional room. Fill with 0xFF.
    let mut game = SnesGame::from_bytes(&bytes);
    game.resize(RomSize::Size4mb);

    // Add base patch to the game. This includes direct fixes to the game.
    for (address, data) in get_patch_data() {
        game.write_all(address, &data);
    }

    // Declare areas that can be used for free space when performing writes.
    for (bank, start, end) in get_free_space() {
        game.mark(bank, start, end);
    }
    game
}

fn get_free_space() -> Vec<(u8, u16, u16)> {
    vec![
        (0x02, 0xFFC7, 0xFFFF),         // Empty Space
        (0x03, 0xEB8F, 0xFFFF),         // Initially contains Doors and Layout.
        (0x09, 0xCB42, 0xEC9C - 0x300), // Contains Sprites in OW and UW, reserve $300 space at the end for the UW pointer table.
        (0x0A, 0x8000, 0xB74F),         // Initially Contains Doors and Layout.
        (0x0E, 0xFD7E, 0xFF9F),         // Empty Space (be careful here, after this is game)
        (0x0F, 0xF4F0, 0xF77F),         // Empty Space
        (0x18, 0xBAE1, 0xBC00),         // Empty Space.
        (0x1B, 0xB1D7, 0xB7FF),         // Empty Space.
        (0x1C, 0xF3D5, 0xF500),         // Empty Space.
        (0x1C, 0xFD8E, 0xFFFF),         // Empty Space.
        (0x1F, 0x878A, 0xFFFF),         // Initially Contains Doors and Layout.
    ]
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use common::Diff;
    use rand::rngs::StdRng;
    use rand::RngCore;
    use rand::SeedableRng;

    use crate::zelda3::features::apply_features;
    use crate::zelda3::io::read_model;
    use crate::zelda3::model::get_spritesheet_arrangements;
    use crate::zelda3::model::OWRoomId;
    use crate::zelda3::model::SpriteId;
    use crate::zelda3::model::SpritesetId;
    use crate::zelda3::model::SpritesheetId;
    use crate::zelda3::model::Z3Model;
    use crate::zelda3::options::Balancing;
    use crate::zelda3::options::OverworldEnemyShuffle;
    use crate::zelda3::options::UnderworldEnemyShuffle;
    use crate::zelda3::options::Z3Options;
    use crate::zelda3::randomize::create_game;
    use crate::zelda3::randomize_zelda3;

    #[test]
    #[ignore]
    fn randomization_causes_deltas() {
        let original = get_file_as_byte_vec("./.testdata/p8.sfc");
        let options = Z3Options {
            seed: "kholdstare".to_owned(),
            overworld_balancing: Balancing::Balanced,
            overworld_enemy_shuffle: OverworldEnemyShuffle::Chaos,
            underworld_balancing: Balancing::Balanced,
            boss_shuffle: true,
            underworld_enemy_shuffle: UnderworldEnemyShuffle::Full,
            ..Z3Options::default()
        };
        let bytes = randomize_zelda3(&original, &options);

        let diffs = Diff::compare(&original, &bytes);
        assert_ne!(diffs.len(), 0);
    }

    #[test]
    #[ignore]
    fn zero_deltas_for_exact_options() {
        // This is manual test that can be run on
        let original = get_file_as_byte_vec("./.testdata/p8.sfc");
        let options = Z3Options {
            seed: "test".to_owned(),
            underworld_balancing: Balancing::Random,
            overworld_balancing: Balancing::Random,
            mushroom_shuffle: true,
            shadow_bees: true,
            overworld_enemy_shuffle: OverworldEnemyShuffle::Full,
            underworld_enemy_shuffle: UnderworldEnemyShuffle::Full,
            boss_shuffle: true,
            ..Z3Options::default()
        };
        let bytes1 = randomize_zelda3(&original, &options);
        let bytes2 = randomize_zelda3(&original, &options);

        let diffs = Diff::compare(&bytes1, &bytes2);
        assert_eq!(diffs.len(), 0);
    }

    #[test]
    #[ignore]
    fn verify_sprites_vanilla() {
        verify_all_sprites_renderable(Z3Options {
            overworld_balancing: Balancing::Random,
            overworld_enemy_shuffle: OverworldEnemyShuffle::Vanilla,
            underworld_balancing: Balancing::Random,
            boss_shuffle: false,
            underworld_enemy_shuffle: UnderworldEnemyShuffle::Vanilla,
            killable_thieves: true,
            mushroom_shuffle: true,
            shadow_bees: true,
            ..Z3Options::default()
        })
    }

    #[test]
    #[ignore]
    fn verify_sprites_full() {
        verify_all_sprites_renderable(Z3Options {
            overworld_balancing: Balancing::Random,
            overworld_enemy_shuffle: OverworldEnemyShuffle::Full,
            underworld_balancing: Balancing::Random,
            boss_shuffle: false,
            underworld_enemy_shuffle: UnderworldEnemyShuffle::Full,
            killable_thieves: true,
            mushroom_shuffle: true,
            shadow_bees: true,
            ..Z3Options::default()
        })
    }

    #[test]
    #[ignore]
    fn verify_sprites_chaos() {
        verify_all_sprites_renderable(Z3Options {
            overworld_balancing: Balancing::Random,
            overworld_enemy_shuffle: OverworldEnemyShuffle::Chaos,
            underworld_balancing: Balancing::Random,
            boss_shuffle: false,
            underworld_enemy_shuffle: UnderworldEnemyShuffle::Chaos,
            killable_thieves: true,
            mushroom_shuffle: true,
            shadow_bees: true,
            ..Z3Options::default()
        })
    }

    #[test]
    #[ignore]
    fn verify_sprites_maximum() {
        verify_all_sprites_renderable(Z3Options {
            overworld_balancing: Balancing::Random,
            overworld_enemy_shuffle: OverworldEnemyShuffle::Chaos,
            underworld_balancing: Balancing::Random,
            boss_shuffle: true,
            underworld_enemy_shuffle: UnderworldEnemyShuffle::Chaos,
            killable_thieves: true,
            mushroom_shuffle: true,
            shadow_bees: true,
            ..Z3Options::default()
        })
    }

    fn verify_all_sprites_renderable(base_options: Z3Options) {
        for seed in generate_seeds() {
            let mut options = base_options.clone();
            options.seed = seed;

            let mut model = get_test_model();
            apply_features(&mut model, &options);

            for room in model.ow_rooms.values() {
                for version in room.versions() {
                    let mut sprite_ids = version.sprites.iter().map(|it| it.id).collect::<Vec<_>>();

                    if room.id == OWRoomId::x1B_HYRULE_CASTLE {
                        // TODO: It seems like Z3Randomizer inserts Sahasrahla at Hyrule Castle, but draws it manually.
                        // need to verify that sahasrahla wasn't moved to a spritesheet instead of being loaded from memory.
                        sprite_ids.retain(|id| *id != SpriteId::x16_SAHASRAHLA);
                    }

                    assert_sprite_configuration_is_valid(
                        &model,
                        version.spriteset_id,
                        &sprite_ids,
                        format!(
                            "Seed: {} | OWRoom: {} | Version: {}",
                            options.seed, room.id, version.overworld_id
                        ),
                    );
                }
            }

            for room in model.uw_sprites.values() {
                let sprite_ids = room.sprites.iter().map(|it| it.id).collect::<Vec<_>>();
                let header = model.uw_headers.get(&room.room_id).unwrap();

                assert_sprite_configuration_is_valid(
                    &model,
                    header.spriteset_id,
                    &sprite_ids,
                    format!("Seed: {} | UWRoom: {}", options.seed, &room.room_id),
                );
            }
        }
    }

    fn generate_seeds() -> Vec<String> {
        // Start with explicit list of tests.
        let mut seeds: Vec<String> = vec![
            "test".to_owned(),
            "saofinsdofinsdofinsodifnsoidf".to_owned(),
        ];
        let mut rng = StdRng::seed_from_u64(0);
        for i in 0..100 {
            seeds.push(format!("SEED_{}_{}", i, rng.next_u64()));
        }

        seeds
    }

    fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
        let mut f = File::open(filename).expect("no file found");
        let metadata = std::fs::metadata(filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        buffer
    }

    fn get_test_model() -> Z3Model {
        let bytes = get_file_as_byte_vec("./.testdata/p8.sfc");
        let game = create_game(&bytes);
        read_model(&game)
    }

    fn assert_sprite_configuration_is_valid(
        model: &Z3Model,
        spriteset_id: SpritesetId,
        sprite_ids: &[SpriteId],
        message: String,
    ) -> bool {
        let spriteset = model.spritesets.get(&spriteset_id).unwrap();
        for sprite_id in sprite_ids {
            let arrangements = get_spritesheet_arrangements(sprite_id);
            if arrangements.is_empty() {
                // Skip verification of in memory sprites.
                continue;
            }

            // Test if this sprite id has some valid arrangement given the spritesheets in the spriteset.
            let has_valid_arrangement = arrangements.iter().any(|spritesheets| {
                spritesheets
                    .iter()
                    .enumerate()
                    .all(|(i, id)| *id == SpritesheetId::None || spriteset.sheets[i] == *id)
            });

            if !has_valid_arrangement {
                panic!(
                    "Invalid sprite config -- Spriteset: ${} | SpriteSheets: {:?} | Sprite: {} | {}",
                    spriteset.id, spriteset.sheets, sprite_id, message
                );
            }
        }
        return true;
    }
}
