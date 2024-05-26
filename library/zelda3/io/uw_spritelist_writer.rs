use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use common::SnesGame;

use crate::zelda3::model::Sprite;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UWSpriteList;
use crate::zelda3::Addresses;

const STOP_MARKER: u8 = 0xFF;
const _OVERLORD_OFFSET: u16 = 0x100;
const SMALL_KEY_MARKER: u8 = 0xFE;
const BIG_KEY_MARKER: u8 = 0xFD;
const BANK: u8 = 0x09;

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

    let mut pointer_map: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
    for (room_id, sprite_list) in spritelists.iter() {
        let bytes = sprite_list_to_bytes(sprite_list);

        let pointer = match pointer_map.entry(bytes.to_owned()) {
            Entry::Vacant(it) => {
                let spritelist_pointer = game
                    .write_data(BANK, &bytes)
                    .expect("Could not write room header");
                *it.insert(spritelist_pointer)
            }
            Entry::Occupied(val) => *val.get(),
        };

        // Write on top of where the sprites used to start. The room sprite pointers are moved in
        // front of all sprites so the overworld and underworld can share space.
        game.write_pointer_int16(
            addresses.room_sprites_end - 0x300 + (*room_id as usize * 2),
            pointer,
        );
    }
}

fn sprite_list_to_bytes(sprite_list: &UWSpriteList) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![];

    if !sprite_list.sorted || sprite_list.sprites.is_empty() {
        buffer.push(0);
    } else {
        buffer.push(1);
    }

    // Rewrite new Dungeon Sprites.
    for sprite in sprite_list.sprites.iter() {
        let bytes = sprite_to_bytes(sprite);
        buffer.extend(bytes);
    }

    // Add the stop marker.
    buffer.push(STOP_MARKER);
    buffer
}

fn sprite_to_bytes(sprite: &Sprite) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![];
    let lower_layer_bit = match sprite.is_lower_layer {
        true => 0b1000_0000,
        _ => 0,
    };
    let overlord_bits = match sprite.id as u16 >= _OVERLORD_OFFSET {
        true => _OVERLORD_OFFSET,
        false => 0,
    } as u8;
    let aux1 = match sprite.id as u16 >= _OVERLORD_OFFSET {
        true => 0b111,
        false => match sprite.aux1 {
            Some(aux1) => aux1 & 0b111,
            _ => 0,
        },
    };

    let y_byte = lower_layer_bit
        | (match sprite.aux0 {
            Some(aux0) => aux0 & 0b11,
            _ => 0,
        })
        | sprite.y;
    let x_byte = sprite.x | (aux1 << 5);
    let sprite_byte = sprite.id as u8 - overlord_bits;

    buffer.extend(&[y_byte, x_byte, sprite_byte]);

    if let Some(item) = sprite.item {
        let marker = match item {
            SpriteId::xE5_BIG_KEY => BIG_KEY_MARKER,
            SpriteId::xE4_SMALL_KEY => SMALL_KEY_MARKER,
            _ => panic!("Non-key sprite found on underworld sprite"),
        };
        buffer.extend(&[marker, 0, 0xE4]);
    }
    buffer
}
