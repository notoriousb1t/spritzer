use std::collections::HashMap;
use assembly::zelda3::Symbol;

use crate::common::readerwriter::WriteObject;
use crate::snes::SnesGame;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UWSpriteList;

const STOP_MARKER: u8 = 0xFF;
const _OVERLORD_OFFSET: u16 = 0x100;
const SMALL_KEY_MARKER: u8 = 0xFE;
const BIG_KEY_MARKER: u8 = 0xFD;

impl WriteObject<HashMap<UWRoomId, UWSpriteList>> for SnesGame {
    fn write_objects(&mut self, spritelists: &HashMap<UWRoomId, UWSpriteList>) {
        // Move the room header references to point to the new location.
        self.write_pointer_int16(
            Symbol::RoomData_SpritePointers_Ref0.into(),
            Symbol::RoomSpritesStart.into(),
        );

        let mut rooms = spritelists.values().collect::<Vec<_>>();
        rooms.sort_by_key(|it| it.uw_room_id);

        for room in rooms.iter() {
            _write_sprites(self, room);
        }
    }
}

fn _write_sprites(game: &mut SnesGame, room: &UWSpriteList) {
    let mut buffer: Vec<u8> = vec![];

    // Sort sprites by their Coordinates to make the ordering more stable.
    let mut sprites = room.sprites.iter().collect::<Vec<_>>();
    sprites.sort_by_key(|it| (it.y_pos, it.x_pos));

    // Rewrite new Dungeon Sprites.
    for dungeon_sprite in sprites.iter() {
        let lower_layer_bit = match dungeon_sprite.lower_layer {
            true => 0b1000_0000,
            false => 0,
        };
        let overlord_bits = match dungeon_sprite.id as u16 >= _OVERLORD_OFFSET {
            true => _OVERLORD_OFFSET,
            false => 0,
        } as u8;
        let aux1 = match dungeon_sprite.id as u16 >= _OVERLORD_OFFSET {
            true => 0b111,
            false => dungeon_sprite.aux1 & 0b111,
        };

        let y_byte = lower_layer_bit | ((dungeon_sprite.aux0 & 0b11) << 5) | dungeon_sprite.y_pos;
        let x_byte = dungeon_sprite.x_pos | (aux1 << 5);
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
            game.write_data(&[0x09], &buffer).unwrap()
        }
        false => Symbol::RoomEmpty as usize,
    };

    // Write on top of where the sprites used to start. The room sprite pointers are moved in
    // front of all sprites so the overworld and underworld can share space.
    game.write_pointer_int16(
        Symbol::RoomSpritesStart as usize + (room.uw_room_id as usize * 2),
        sprites_location,
    );
}
