use std::collections::HashMap;
use assembly::zelda3::Symbol;
use strum::IntoEnumIterator;

use crate::common::readerwriter::ReadObject;
use crate::snes::SnesGame;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UWSprite;
use crate::zelda3::model::UWSpriteList; // Required for EnumIter::iter().

const STOP_MARKER: u8 = 0xFF;
const _OVERLORD_OFFSET: u16 = 0x100;
const SMALL_KEY_MARKER: u8 = 0xFE;
const BIG_KEY_MARKER: u8 = 0xFD;

impl ReadObject<HashMap<UWRoomId, UWSpriteList>> for SnesGame {
    fn read_objects(&self) -> HashMap<UWRoomId, UWSpriteList> {
        let mut values: Vec<(UWRoomId, UWSpriteList)> = vec![];
        for id in UWRoomId::iter() {
            values.push((id, _read_room(self, id)));
        }
        HashMap::from_iter(values)
    }
}

fn _read_room(game: &SnesGame, id: UWRoomId) -> UWSpriteList {
    let base_address = game.read_pointer_int16(Symbol::UwSpritePtrs as usize + (id as usize * 2));
    let layering = game.read(base_address) > 0;
    let sprites = _read_sprites(game, base_address);

    UWSpriteList {
        uw_room_id: id,
        sorted: layering,
        sprites,
    }
}

fn _read_sprites(game: &SnesGame, base_address: usize) -> Vec<UWSprite> {
    let mut index = 1; // byte 0 handles sprite ordering.
    let mut underworld_sprites: Vec<UWSprite> = vec![];

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

        underworld_sprites.push(UWSprite {
            id: sprite_id,
            y_pos: y,
            x_pos: x,
            lower_layer,
            aux0,
            aux1,
            item,
            distance_from_midpoint: 0,
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
