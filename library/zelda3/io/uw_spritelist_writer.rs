use std::collections::BTreeMap;

use common::SnesGame;

use crate::zelda3::model::SpriteId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UWSpriteList;
use crate::zelda3::Addresses;

const STOP_MARKER: u8 = 0xFF;
const _OVERLORD_OFFSET: u16 = 0x100;
const SMALL_KEY_MARKER: u8 = 0xFE;
const BIG_KEY_MARKER: u8 = 0xFD;
const MOVED_BANK: u8 = 0x09;

pub(super) fn write_uw_spritelists(
    game: &mut SnesGame,
    addresses: &Addresses,
    spritelists: &BTreeMap<UWRoomId, UWSpriteList>,
) {
    // Move the room header references to point to the new location.
    game.write_pointer_int16(
        addresses.room_data_sprite_pointers_ref0,
        addresses.room_sprites_end - 0x300,
    );
    for room in spritelists.values() {
        _write_sprites(game, addresses, room);
    }
}

fn _write_sprites(game: &mut SnesGame, addresses: &Addresses, room: &UWSpriteList) {
    let mut buffer: Vec<u8> = vec![];

    // Rewrite new Dungeon Sprites.
    for dungeon_sprite in room.sprites.iter() {
        let lower_layer_bit = match dungeon_sprite.is_lower_layer {
            true => 0b1000_0000,
            _ => 0,
        };
        let overlord_bits = match dungeon_sprite.id as u16 >= _OVERLORD_OFFSET {
            true => _OVERLORD_OFFSET,
            false => 0,
        } as u8;
        let aux1 = match dungeon_sprite.id as u16 >= _OVERLORD_OFFSET {
            true => 0b111,
            false => match dungeon_sprite.aux1 {
                Some(aux1) => aux1 & 0b111,
                _ => 0,
            },
        };

        let y_byte = lower_layer_bit
            | (match dungeon_sprite.aux0 {
                Some(aux0) => aux0 & 0b11,
                _ => 0,
            })
            | dungeon_sprite.y;
        let x_byte = dungeon_sprite.x | (aux1 << 5);
        let sprite_byte = dungeon_sprite.id as u8 - overlord_bits;

        buffer.extend(&[y_byte, x_byte, sprite_byte]);

        if let Some(item) = dungeon_sprite.item {
            let marker = match item {
                SpriteId::xE5_BIG_KEY => BIG_KEY_MARKER,
                SpriteId::xE4_SMALL_KEY => SMALL_KEY_MARKER,
                _ => panic!("Non-key sprite found on underworld sprite"),
            };
            buffer.extend(&[marker, 0, 0xE4]);
        }
    }

    let sprites_location = match !buffer.is_empty() {
        true => {
            // Add layering to the beginning.
            buffer.insert(
                0,
                match room.sorted {
                    true => 1,
                    false => 0,
                },
            );
            // Add End marker.
            buffer.push(STOP_MARKER);
            game.write_data(MOVED_BANK, &buffer).unwrap()
        }
        false => addresses.uwroom_empty,
    };

    // Write on top of where the sprites used to start. The room sprite pointers are moved in
    // front of all sprites so the overworld and underworld can share space.
    game.write_pointer_int16(
        addresses.room_sprites_end - 0x300 + (room.room_id as usize * 2),
        sprites_location,
    );
}
