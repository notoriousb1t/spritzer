use assembly::zelda3::Symbol;
use std::collections::BTreeMap;

use crate::common::readerwriter::WriteObject;
use crate::snes::bytes_to_address;
use crate::snes::SnesGame;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UnderworldRoomHeader;

impl WriteObject<BTreeMap<UWRoomId, UnderworldRoomHeader>> for SnesGame {
    fn write_objects(&mut self, headers: &BTreeMap<UWRoomId, UnderworldRoomHeader>) {
        let header_16bit_ptr = self.read_all(Symbol::UWHeaderRef0.into(), 2);
        let header_bank = self.read(Symbol::UWHeaderBank.into());
        let header_pointer =
            bytes_to_address([header_bank, header_16bit_ptr[1], header_16bit_ptr[0]]);

        for room in headers.values() {
            _write_metadata(self, header_pointer, room);
        }
    }
}

fn _write_metadata(game: &mut SnesGame, header_pointer: usize, room: &UnderworldRoomHeader) {
    game.write_all(
        game.read_pointer_int16(header_pointer + (room.id as usize * 2)),
        &[
            room.bg2_property,
            room.palette_id as u8,
            room.blockset_id as u8,
            room.spriteset_id.get_room_value(),
            room.bgmove,
            room.tag1 as u8,
            room.tag2 as u8,
            room.planes1 as u8,
            room.planes2 as u8,
            room.warp as u8,
            room.stairs0 as u8,
            room.stairs1 as u8,
            room.stairs2 as u8,
            room.stairs3 as u8,
        ],
    );
}