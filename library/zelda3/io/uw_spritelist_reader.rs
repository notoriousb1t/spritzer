use std::collections::BTreeMap;

use assembly::zelda3::Symbol;
use common::SnesGame;
use strum::IntoEnumIterator;

use crate::zelda3::model::Sprite;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UWSpriteList;

const STOP_MARKER: u8 = 0xFF;
const _OVERLORD_OFFSET: u16 = 0x100;
const SMALL_KEY_MARKER: u8 = 0xFE;
const BIG_KEY_MARKER: u8 = 0xFD;

pub(super) fn read_uw_spritelists(game: &SnesGame) -> BTreeMap<UWRoomId, UWSpriteList> {
    let mut values: Vec<(UWRoomId, UWSpriteList)> = vec![];
    for id in UWRoomId::iter() {
        values.push((id, _read_room(game, id)));
    }
    BTreeMap::from_iter(values)
}

fn _read_room(game: &SnesGame, room_id: UWRoomId) -> UWSpriteList {
    let base_address =
        game.read_pointer_int16(Symbol::UwSpritePtrs as usize + (room_id as usize * 2));
    let sorted = game.read(base_address) > 0;
    let sprites = _read_sprites(game, base_address);

    UWSpriteList {
        room_id,
        sorted,
        sprites,
    }
}

fn _read_sprites(game: &SnesGame, base_address: usize) -> Vec<Sprite> {
    let mut index = 1; // byte 0 handles sprite ordering.
    let mut underworld_sprites: Vec<Sprite> = vec![];

    loop {
        // Read the sprite table for this Dungeon Room.
        let address = base_address + index;
        // Peek to look for 255: the stop character.
        // This happens when there are no more Sprites in the Dungeon Room.
        // More data appears to be after this marker, so this should remain
        // a fixed length.
        if game.read(address) == STOP_MARKER {
            break;
        }
        // Read from byte 0 lssyyyyy
        let byte0 = game.read(address);
        let lower_layer = (byte0 & 0b1000_0000) != 0;
        let aux0 = (byte0 & 0b0110_0000) >> 5;
        let y = byte0 & 0b0001_1111;

        let byte1 = game.read(address + 1);
        let x = byte1 & 0b0001_1111;
        let aux1 = (byte1 & 0b1110_0000) >> 5;

        let sprite_value = game.read(address + 2);
        let sprite_id = match SpriteId::from_repr(
            sprite_value as u16
                + (if aux1 == 0b111 {
                    _OVERLORD_OFFSET
                } else {
                    0_u16
                }),
        ) {
            Some(sprite) => sprite,
            None => {
                panic!("Panicked on {}", sprite_value);
            }
        };
        let item = _check_for_key(game, address);

        underworld_sprites.push(Sprite {
            id: sprite_id,
            y,
            x,
            is_lower_layer: lower_layer,
            aux0: Some(aux0),
            aux1: Some(aux1),
            item,
        });
        if item.is_some() {
            index += 6;
        } else {
            index += 3;
        }
    }
    underworld_sprites
}

fn _check_for_key(game: &SnesGame, sprite_address: usize) -> Option<SpriteId> {
    match game.read(sprite_address + 3) {
        SMALL_KEY_MARKER => Some(SpriteId::xE4_SMALL_KEY),
        BIG_KEY_MARKER => Some(SpriteId::xE5_BIG_KEY),
        _ => None,
    }
}
