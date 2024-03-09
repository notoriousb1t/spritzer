use std::collections::BTreeMap;

use assembly::zelda3::Symbol;
use common::bytes_to_int24;
use common::SnesGame;

use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UnderworldRoomHeader;

pub(super) fn write_uw_headers(
    game: &mut SnesGame,
    headers: &BTreeMap<UWRoomId, UnderworldRoomHeader>,
) {
    let header_16bit_ptr = game.read_all(Symbol::UWHeaderRef0.into(), 2);
    let header_bank = game.read(Symbol::UWHeaderBank.into());
    let header_pointer = bytes_to_int24([header_bank, header_16bit_ptr[1], header_16bit_ptr[0]]);

    for room in headers.values() {
        _write_metadata(game, header_pointer, room);
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
