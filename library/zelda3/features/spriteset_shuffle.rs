use std::collections::btree_map::Entry;

use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use strum::IntoEnumIterator;

use crate::zelda3::model::calculate_sprite_pool;
use crate::zelda3::model::can_shuffle_in_ow;
use crate::zelda3::model::can_shuffle_in_uw;
use crate::zelda3::model::can_shuffle_type;
use crate::zelda3::model::get_overworld_rules;
use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::get_spritesheet_arrangements;
use crate::zelda3::model::get_weights;
use crate::zelda3::model::is_list_compatible;
use crate::zelda3::model::is_special_damage_sprite;
use crate::zelda3::model::is_spritesheet_permanent;
use crate::zelda3::model::DungeonId;
use crate::zelda3::model::OWRoomId;
use crate::zelda3::model::OWStateId;
use crate::zelda3::model::Rule;
use crate::zelda3::model::Sprite;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::Spriteset;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::SpritesheetId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::Z3Model;
use crate::zelda3::Balancing;

pub(super) fn shuffle_overworld_spritesets(model: &mut Z3Model) {
    log::debug!("Shuffling overworld spritesets");
    let rng = &mut model.create_rng();

    // Ensure that all usable spritesets are filled.
    let spriteset_ids = SpritesetId::iter()
        .filter(|it| !it.is_underworld())
        .collect::<Vec<_>>();

    fill_spritesets(
        model,
        spriteset_ids,
        model.ow_balancing,
        &[Rule::Overworld],
        rng,
    );

    let sprite_pool = calculate_sprite_pool(model);

    for room_id in OWRoomId::iter() {
        if is_overworld_room_locked(room_id) {
            log::debug!(
                "Skipping spriteset shuffle because of permanent spriteset. Room = {}",
                room_id
            );
            continue;
        }

        let room = model.ow_rooms.get_mut(&room_id).unwrap();
        for screen in room.versions_mut() {
            if room_id == OWRoomId::x16_WITCHS_HUT
                && screen.overworld_id == OWStateId::DARK_WORLD_V1
            {
                log::info!("here");
            }

            // Pass any spritesets with required spritesheets (turtle rock, somaria platform for
            // example).
            let spriteset_id = screen.spriteset_id;
            let spriteset = model.spritesets.get(&spriteset_id).unwrap();
            if spriteset.sheets.iter().any(is_spritesheet_permanent) {
                log::debug!(
                    "Skipping spriteset shuffle because of permanent spritesheet. Room = {}",
                    room_id
                );
                continue;
            }

            // Pass any rooms which have permanent sprites (objects, npcs, special damage type
            // enemies)
            let rules = get_overworld_rules(screen.overworld_id);
            if has_special_requirements(screen.sprites.clone(), &rules) {
                log::debug!(
                    "Skipping spriteset shuffle because of sprite requirements. Room = {}, {}",
                    room_id,
                    screen.overworld_id
                );
                continue;
            }

            // Compute the list of compatible spritesets.
            let spriteset_pool = sprite_pool
                .iter()
                .filter(|(spriteset_id, sprites)| {
                    !spriteset_id.is_underworld()
                        && is_list_compatible(&screen.sprites, &sprites, &rules)
                })
                .map(|(spriteset_id, _)| *spriteset_id)
                .collect::<Vec<_>>();

            if spriteset_pool.is_empty() {
                log::debug!(
                    "Skipping spriteset shuffle because there are no valid swaps. Room = {}",
                    room_id
                );
                continue;
            }

            if let Some(spriteset_id) = spriteset_pool.choose(rng) {
                log::info!(
                    "Spriteset shuffle success. Room = {}. From = {}, To = {}",
                    room_id,
                    screen.spriteset_id,
                    *spriteset_id
                );
                screen.spriteset_id = *spriteset_id;
            } else {
                log::debug!(
                    "Skipping spriteset shuffle because RNG failed. Room = {}",
                    room_id
                );
            }
        }
    }
}

pub(super) fn shuffle_underworld_spritesets(model: &mut Z3Model) {
    log::debug!("Shuffling underworld spritesets");
    let rng = &mut model.create_rng();

    // Ensure that all usable spritesets are filled.
    let spriteset_ids = SpritesetId::iter()
        .filter(|it| it.is_underworld())
        .collect::<Vec<_>>();

    fill_spritesets(
        model,
        spriteset_ids,
        model.ow_balancing,
        &[Rule::Underworld],
        rng,
    );

    let sprite_pool = calculate_sprite_pool(model);

    for room_id in UWRoomId::iter() {
        if is_underworld_room_locked(room_id) {
            log::debug!(
                "Skipping spriteset shuffle because of permanent spriteset. Room = {}",
                room_id
            );
            continue;
        }

        let header = model.uw_headers.get_mut(&room_id).unwrap();
        let screen = model.uw_sprites.get_mut(&room_id).unwrap();

        // Pass any spritesets with required spritesheets (turtle rock, somaria platform for
        // example).
        let spriteset_id = header.spriteset_id;
        let spriteset = model.spritesets.get(&spriteset_id).unwrap();
        if spriteset.sheets.iter().any(is_spritesheet_permanent) {
            log::debug!(
                "Skipping spriteset shuffle because of permanent spritesheet. Room = {}",
                room_id
            );
            continue;
        }

        // If the room is part of escape or rescue, artificially lower the difficulty because this
        // could be a standard run and there are some rooms that are painful given the
        // number of pests normally there or impossible to kill with just the arrows.
        let is_rescue = model
            .dungeons
            .iter()
            .find(|(dungeon_id, dungeon)| {
                (**dungeon_id == DungeonId::X00_Sewers
                    || **dungeon_id == DungeonId::X02_HyruleCastle)
                    && dungeon.rooms.contains(&room_id)
            })
            .is_some();

        // Pass any rooms which have permanent sprites (objects, npcs, special damage type enemies)
        let mut rules = vec![Rule::Underworld];
        if header.tag1.is_kill_room() || header.tag2.is_kill_room() {
            rules.push(Rule::KillRequired);
        };
        if is_rescue {
            rules.push(Rule::ReduceDifficulty);
        }

        if has_special_requirements(screen.sprites.clone(), &rules) {
            log::debug!(
                "Skipping spriteset shuffle because of sprite requirements. Room = {}",
                room_id
            );
            continue;
        }

        // Compute the list of compatible spritesets.
        let spriteset_pool = sprite_pool
            .iter()
            .filter(|(spriteset_id, sprites)| {
                spriteset_id.is_underworld()
                    && is_list_compatible(&screen.sprites, &sprites, &rules)
            })
            .map(|(spriteset_id, _)| *spriteset_id)
            .collect::<Vec<_>>();

        if spriteset_pool.is_empty() {
            log::debug!(
                "Skipping spriteset shuffle because there are no valid swaps. Room = {}",
                room_id
            );
            continue;
        }

        if let Some(spriteset_id) = spriteset_pool.choose(rng) {
            log::info!(
                "Spriteset shuffle success. Room = {}. From = {}, To = {}",
                room_id,
                header.spriteset_id,
                *spriteset_id
            );
            header.spriteset_id = *spriteset_id;
        } else {
            log::debug!(
                "Skipping spriteset shuffle because RNG failed. Room = {}",
                room_id
            );
        }
    }
}

fn is_overworld_room_locked(room_id: OWRoomId) -> bool {
    match room_id {
        OWRoomId::x40_MASTER_SWORD_UNDER_BRIDGE => true,
        _ => false,
    }
}

fn is_underworld_room_locked(room_id: UWRoomId) -> bool {
    match room_id {
        UWRoomId::x61_HYRULE_CASTLE_MAIN_ENTRANCE_ROOM => true,
        UWRoomId::x62_HYRULE_CASTLE_EAST_ENTRANCE_ROOM => true,
        UWRoomId::xDB_THIEVES_TOWN_MAIN_SOUTH_WEST_ENTRANCE_ROOM => true,
        _ => false,
    }
}

fn fill_spritesets(
    model: &mut Z3Model,
    spritesets: Vec<SpritesetId>,
    balancing: Balancing,
    rules: &[Rule],
    rng: &mut StdRng,
) {
    for spriteset_id in spritesets.iter() {
        let spriteset = match model.spritesets.entry(*spriteset_id) {
            Entry::Vacant(it) => it.insert(Spriteset {
                id: *spriteset_id,
                sheets: [SpritesheetId::None; 4],
            }),
            Entry::Occupied(it) => it.into_mut(),
        };
        let sheets = fill_spriteset(spriteset.sheets.clone(), balancing, rules, rng);
        spriteset.sheets = sheets;
    }
}

fn fill_spriteset(
    initial_spritesheets: [SpritesheetId; 4],
    balancing: Balancing,
    rules: &[Rule],
    rng: &mut StdRng,
) -> [SpritesheetId; 4] {
    let mut updated_spritesheets = initial_spritesheets.clone();

    // Attempt at most 16 times to find replacements for empty spritesheets.
    for counter in 0..16 {
        if is_spriteset_full(updated_spritesheets) {
            return updated_spritesheets;
        }

        for sprite_type in [SpriteType::Enemy, SpriteType::Hazard, SpriteType::Creature] {
            // Determine how many of this type of sprite are possible with the current spritesheets.
            let matching_type_count = get_possible_sprites(updated_spritesheets, rules)
                .iter()
                .filter(|sprite_id| get_sprite_type(sprite_id) == sprite_type)
                .count();
            if matching_type_count > counter {
                // If there are enough of this type of Sprite, continue to the next type.
                continue;
            }

            let potential_spritesheets =
                get_possible_spritesheets(updated_spritesheets, sprite_type, rules);
            if potential_spritesheets.is_empty() {
                continue;
            }

            let mut sprite_pool = potential_spritesheets
                .iter()
                .map(|(sprite_id, _)| *sprite_id)
                .collect::<Vec<_>>();
            sprite_pool.sort();
            sprite_pool.dedup();

            let weights = get_weights(&balancing, rules, &sprite_pool);

            match potential_spritesheets.choose_weighted(rng, |(sprite_id, _)| weights[sprite_id]) {
                Ok((_, replacement)) => {
                    for i in 0..4 {
                        if updated_spritesheets[i] == SpritesheetId::None
                            && replacement[i] != SpritesheetId::None
                        {
                            updated_spritesheets[i] = replacement[i];
                        }
                    }
                },
                Err(msg) => {
                    panic!("{}", msg);
                },
            }

            if is_spriteset_full(updated_spritesheets) {
                return updated_spritesheets;
            }
        }
    }

    updated_spritesheets
}

fn is_spriteset_full(spritesheets: [SpritesheetId; 4]) -> bool {
    spritesheets
        .iter()
        .all(|spritesheet| *spritesheet != SpritesheetId::None)
}

fn get_possible_spritesheets(
    current_spritesheets: [SpritesheetId; 4],
    sprite_type: SpriteType,
    rules: &[Rule],
) -> Vec<(SpriteId, [SpritesheetId; 4])> {
    let filter = if rules.contains(&Rule::Overworld) {
        can_shuffle_in_ow
    } else if rules.contains(&Rule::Overworld) {
        can_shuffle_in_uw
    } else {
        can_shuffle_type
    };

    SpriteId::iter()
        .filter(|sprite_id| get_sprite_type(sprite_id) == sprite_type && filter(sprite_id))
        .flat_map(|sprite_id| {
            let arrangements = get_spritesheet_arrangements(&sprite_id);
            arrangements
                .iter()
                .map(|ss| (sprite_id, *ss))
                .filter(|(_, arrangement)| {
                    arrangement.iter().enumerate().all(|(index, spritesheet)| {
                        current_spritesheets[index] == SpritesheetId::None
                            || *spritesheet == SpritesheetId::None
                            || current_spritesheets[index] == *spritesheet
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_possible_sprites(spritesheets: [SpritesheetId; 4], rules: &[Rule]) -> Vec<SpriteId> {
    let filter = if rules.contains(&Rule::Overworld) {
        can_shuffle_in_ow
    } else if rules.contains(&Rule::Overworld) {
        can_shuffle_in_uw
    } else {
        can_shuffle_type
    };

    SpriteId::iter()
        .filter(filter)
        .map(|sprite_id| (sprite_id, get_spritesheet_arrangements(&sprite_id)))
        .filter(|(_, arrangements)| {
            arrangements.iter().any(|arrangement| {
                arrangement.iter().enumerate().all(|(index, spritesheet)| {
                    *spritesheet == SpritesheetId::None || spritesheets[index] == *spritesheet
                })
            })
        })
        .map(|(sprite_id, _)| sprite_id)
        .collect::<Vec<_>>()
}

fn has_special_requirements(sprites: Vec<Sprite>, rules: &[Rule]) -> bool {
    sprites
        .iter()
        .map(|sprite| sprite.id)
        .any(|sprite_id| is_required_sprite(&sprite_id, rules))
}

fn is_required_sprite(sprite_id: &SpriteId, rules: &[Rule]) -> bool {
    match get_sprite_type(sprite_id) {
        SpriteType::Npc => true,
        SpriteType::Object => true,
        SpriteType::Boss => true,
        SpriteType::Overlord => true,
        SpriteType::Enemy => {
            if rules.contains(&Rule::KillRequired) {
                is_special_damage_sprite(sprite_id)
            } else {
                false
            }
        }
        _ => false,
    }
}
