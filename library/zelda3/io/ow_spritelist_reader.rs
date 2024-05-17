use core::panic;
use std::collections::BTreeMap;

use common::SnesGame;
use strum::IntoEnumIterator;

use super::ow_spritelist_utils::get_palette_address;
use super::ow_spritelist_utils::get_sprite_graphics_address;
use super::ow_spritelist_utils::get_sprite_pointer;
use crate::zelda3::model::OWRoom;
use crate::zelda3::model::OWRoomId;
use crate::zelda3::model::OWRoomState;
use crate::zelda3::model::OWStateId;
use crate::zelda3::model::Sprite;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpritesetId;

const STOP_MARKER: u8 = 0xFF;

/// Returns OW Sprite List for each OW Room.
pub(super) fn read_ow_sprites_and_headers(game: &SnesGame) -> BTreeMap<OWRoomId, OWRoom> {
    let mut values: Vec<(OWRoomId, OWRoom)> = vec![];
    for id in OWRoomId::iter() {
        values.push((id, read_room(game, id)));
    }
    BTreeMap::from_iter(values)
}

fn read_room(game: &SnesGame, id: OWRoomId) -> OWRoom {
    let lw_v0: Option<OWRoomState> = match id {
        OWRoomId::x1B_HYRULE_CASTLE => Some(read_room_state(game, id, OWStateId::LIGHT_WORLD_V0)),
        OWRoomId::x2B_FOREST_BETWEEN_HAUNTED_GROVE_AND_LINKS_HOUSE => {
            Some(read_room_state(game, id, OWStateId::LIGHT_WORLD_V0))
        }
        OWRoomId::x2C_LINKS_HOUSE => Some(read_room_state(game, id, OWStateId::LIGHT_WORLD_V0)),
        _ => None,
    };

    let lw_v1 = read_room_state(game, id, OWStateId::LIGHT_WORLD_V1);

    let lw_post_aga = match id {
        OWRoomId::x1B_HYRULE_CASTLE => Some(read_room_state(game, id, OWStateId::LIGHT_WORLD_V2)),
        OWRoomId::x2_LUMBER_JACK_HOUSE => {
            Some(read_room_state(game, id, OWStateId::LIGHT_WORLD_V2))
        }
        _ => None,
    };
    let dw: Option<OWRoomState> = match id {
        OWRoomId::x40_MASTER_SWORD_UNDER_BRIDGE => None,
        OWRoomId::x41_ZORAS_DOMAIN => None,
        _ => Some(read_room_state(game, id, OWStateId::DARK_WORLD_V1)),
    };

    OWRoom {
        id,
        lw_rescue: lw_v0,
        lw: lw_v1,
        lw_post_aga,
        dw,
    }
}

// Reads an Area from the ROM and returns it as a data class.
fn read_room_state(game: &SnesGame, id: OWRoomId, overworld_id: OWStateId) -> OWRoomState {
    // Resolve the sprite graphics and sprite palette id.
    let spriteset_id_value = game.read(get_sprite_graphics_address(id, overworld_id));
    let spriteset_id = SpritesetId::from_repr(spriteset_id_value)
        .expect(&format!("Invalid Spriteset {}", spriteset_id_value));
    let sprite_palette_id = game.read(get_palette_address(id, overworld_id));
    let sprites = read_sprites(game, id, overworld_id);

    OWRoomState {
        overworld_id,
        spriteset_id,
        sprite_palette_id,
        sprites,
    }
}

fn read_sprites(
    game: &SnesGame,
    overworld_area_id: OWRoomId,
    overworld_id: OWStateId,
) -> Vec<Sprite> {
    let sprite_address = get_sprite_pointer(overworld_area_id, overworld_id);
    // Find the base address of Overworld Sprites in this Overworld Area.
    let sprite_table_base_address = game.read_pointer_int16(sprite_address);

    let mut sprites: Vec<Sprite> = vec![];
    let mut index = 0;
    let mut remaining_max_bytes = 10000;
    loop {
        // Read the sprite table for this Overworld Area.
        let address: usize = sprite_table_base_address + index;
        // Peek to look for 255: the stop character.
        // This happens when there are no more Overworld Sprites in the Overworld Area.
        // More data appears to be after this marker, so this should remain
        // a fixed length.
        if game.read(address) == STOP_MARKER {
            break;
        }
        if remaining_max_bytes == 0 {
            panic!("Maximum bytes exceeded. Aborting to prevent infinite loop");
        }
        let y = game.read(address);
        let x = game.read(address + 1);
        let sprite_value = game.read(address + 2);
        let sprite_id = SpriteId::from_repr(sprite_value as u16).unwrap();
        sprites.push(Sprite {
            y,
            x,
            id: sprite_id,
            is_lower_layer: false,
            item: None,
            aux0: None,
            aux1: None,
        });
        index += 3;
        remaining_max_bytes -= 3;
    }
    sprites
}
