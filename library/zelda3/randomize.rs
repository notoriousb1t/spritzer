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
    let mut game = create_game(bytes);
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

    use crate::zelda3::options::Balancing;
    use crate::zelda3::options::OverworldEnemyShuffle;
    use crate::zelda3::options::UnderworldEnemyShuffle;
    use crate::zelda3::options::Z3Options;
    use crate::zelda3::randomize_zelda3;

    #[test]
    #[ignore]
    fn randomization_causes_deltas() {
        let original = get_file_as_byte_vec("./.testdata/base.sfc");
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
        let original = get_file_as_byte_vec("./.testdata/base.sfc");
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

    fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
        let mut f = File::open(filename).expect("no file found");
        let metadata = std::fs::metadata(filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        buffer
    }
}
