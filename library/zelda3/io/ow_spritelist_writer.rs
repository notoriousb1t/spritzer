use std::collections::hash_map::Entry;
use std::collections::BTreeMap;
use std::collections::HashMap;

use assembly::zelda3::Symbol;
use common::SnesGame;

use super::ow_spritelist_utils::get_palette_address;
use super::ow_spritelist_utils::get_sprite_graphics_address;
use super::ow_spritelist_utils::get_sprite_pointer;
use crate::zelda3::model::OWRoom;
use crate::zelda3::model::OWRoomId;
use crate::zelda3::model::OWSprite;
use crate::zelda3::model::OWStateId;

const STOP_MARKER: u8 = 0xFF;

pub(super) fn write_ow_sprites_and_headers(
    game: &mut SnesGame,
    rooms: &BTreeMap<OWRoomId, OWRoom>,
) {
    let mut rooms = rooms.values().collect::<Vec<_>>();
    rooms.sort_by_key(|room| room.id);

    write_headers(game, &rooms);
    write_spritelists(game, &rooms);
}

fn write_headers(game: &mut SnesGame, rooms: &[&OWRoom]) {
    for room in rooms.iter() {
        for state in room.states() {
            for overworld_id in get_affected_state_ids(room, state.overworld_id) {
                game.write(
                    get_sprite_graphics_address(room.id, overworld_id),
                    state.spriteset_id as u8,
                );
                game.write(
                    get_palette_address(room.id, overworld_id),
                    state.sprite_palette_id,
                );
            }
        }
    }
}

fn write_spritelists(game: &mut SnesGame, rooms: &[&OWRoom]) {
    // Group room + state by sprites since identical sprite lists can be written as a single one
    // and pointed to by multiple states.
    let mut map: HashMap<Vec<OWSprite>, Vec<(OWRoomId, OWStateId)>> = HashMap::default();
    for room in rooms.iter() {
        for state in room.states() {
            let values = match map.entry(state.sprites.clone()) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(vec![]),
            };
            for overworld_id in get_affected_state_ids(room, state.overworld_id) {
                values.push((room.id, overworld_id));
            }
        }
    }

    let mut sorted_entries = map.iter().collect::<Vec<_>>();
    sorted_entries.sort_by_key(|it| it.1[0]);

    for (sprites, update_list) in sorted_entries {
        // Start building the bytes for this overworld sprite list.
        let mut bytes: Vec<u8> = vec![];

        // Rewrite Overworld Sprites back into the same spots.
        let mut sorted_sprites = sprites.iter().collect::<Vec<_>>();
        sorted_sprites.sort_by_key(|it| (it.y, it.x));

        for sprite in sorted_sprites {
            bytes.extend(&[sprite.y, sprite.x, sprite.id as u8]);
        }

        // If there are no sprites loaded, point to the empty room offset by 1. The Overworld
        // uses a byte format of (Y X SpriteId)+ terminated by 0xFF. The Underworld
        // uses a format of OAMLayering (Y&overlord X&aux SpriteId) terminated by
        // 0xFF, so the overworld can use The empty room if it starts after the
        // OAMLayering byte.
        let sprites_location = match !bytes.is_empty() {
            true => {
                bytes.push(STOP_MARKER);
                game.write_data(&[0x09], &bytes).unwrap()
            }
            false => Symbol::OWRoomEmpty.into(),
        };

        for (room_id, ow_id) in update_list.iter() {
            game.write_pointer_int16(get_sprite_pointer(*room_id, *ow_id), sprites_location);
        }
    }
}

/// Returns the affected overworld state ids based on the area and state id being set.
fn get_affected_state_ids(area: &OWRoom, overworld_id: OWStateId) -> Vec<OWStateId> {
    match overworld_id {
        OWStateId::LIGHT_WORLD_V0 => vec![OWStateId::LIGHT_WORLD_V0],
        OWStateId::LIGHT_WORLD_V1 => {
            if area.id == OWRoomId::x40_MASTER_SWORD_UNDER_BRIDGE
                || area.id == OWRoomId::x41_ZORAS_DOMAIN
            {
                return vec![OWStateId::LIGHT_WORLD_V1, OWStateId::LIGHT_WORLD_V2];
            }

            let mut light_overworld_ids: Vec<OWStateId> = vec![];
            if area.lw_rescue.is_none() {
                light_overworld_ids.push(OWStateId::LIGHT_WORLD_V0);
            }
            light_overworld_ids.push(OWStateId::LIGHT_WORLD_V1);
            if area.lw_post_aga.is_none() {
                light_overworld_ids.push(OWStateId::LIGHT_WORLD_V2);
            }
            light_overworld_ids
        }
        OWStateId::LIGHT_WORLD_V2 => vec![OWStateId::LIGHT_WORLD_V2],
        OWStateId::DARK_WORLD_V2 => {
            vec![OWStateId::DARK_WORLD_V1, OWStateId::DARK_WORLD_V2]
        }
        _ => vec![],
    }
}
