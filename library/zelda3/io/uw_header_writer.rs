use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use assembly::zelda3::Symbol;
use common::SnesGame;

use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UnderworldRoomHeader;

const MOVED_HEADER_BANK: u8 = 0x2A;

pub(super) fn write_uw_headers(
    game: &mut SnesGame,
    headers: &BTreeMap<UWRoomId, UnderworldRoomHeader>,
) {
    // Write the data first and try to collapse header with complete overlaps
    let mut header_pointer_map: BTreeMap<[u8; 14], usize> = BTreeMap::new();
    let mut pointers = vec![];
    for room in headers.values() {
        // Write the data to the next available place in this bank.
        let bytes = &header_to_bytes(&room);

        // If the exact header exists already, reuse the existing pointer.
        let pointer = match header_pointer_map.entry(*bytes) {
            Entry::Vacant(it) => {
                let room_header_pointer = game
                    .write_data(&[MOVED_HEADER_BANK], bytes)
                    .expect("Could not write room header");
                *it.insert(room_header_pointer)
            }
            Entry::Occupied(val) => {
                *val.get()
            }
        };

        // Add the pointer to the list, so they can be written after all data
        // is writtern.
        pointers.push(pointer);
    }
    
    // Allocate enough space to store the pointers.
    let room_header_table_pointer = game
        .allocate(MOVED_HEADER_BANK, 2 * pointers.len() as u16)
        .expect("Could not find freespace to write Underworld Header Table");

    // Update the 2 references to the UW header pointer table.
    game.write(Symbol::UWHeaderBank.into(), MOVED_HEADER_BANK);
    game.write_pointer(Symbol::UWHeaderRef0.into(), room_header_table_pointer);

    // Write the pointers table.
    for (i, pointer) in pointers.iter().enumerate() {
        game.write_pointer_int16(
            room_header_table_pointer + (i * 2),
            *pointer,
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
