use std::collections::BTreeMap;

use assembly::zelda3::Symbol;
use common::SnesGame;

use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UnderworldRoomHeader;

const MOVED_HEADER_BANK: u8 = 0x36;
const TOTAL_ROOM_COUNT: u16 = 0x140;

pub(super) fn write_uw_headers(
    game: &mut SnesGame,
    headers: &BTreeMap<UWRoomId, UnderworldRoomHeader>,
) {
    // Allocate enough space to write pointers for $140 local pointers.
    let room_header_table_pointer = game
        .allocate(MOVED_HEADER_BANK, 2 * TOTAL_ROOM_COUNT)
        .expect("Could not find freespace to write Underworld Header Table");

    // Write new pointer values to 0x8000 to reference start of room headers.
    game.write(Symbol::UWHeaderBank.into(), MOVED_HEADER_BANK);
    game.write_pointer(Symbol::UWHeaderRef0.into(), room_header_table_pointer);

    for room in headers.values() {
        // Write the data to the next available place in this bank.
        let room_header_pointer = game
            .write_data(&[MOVED_HEADER_BANK], &header_to_bytes(&room))
            .expect("Could not write room header");

        // Write the pointer to the pointer table.
        game.write_pointer_int16(
            room_header_table_pointer + (room.id as usize * 2),
            room_header_pointer,
        );
    }
}

fn header_to_bytes(room: &UnderworldRoomHeader) -> [u8; 14] {
    [
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
    ]
}
