use assembly::zelda3::Symbol;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

use crate::common::readerwriter::ReadObject;
use crate::snes::SnesGame;
use crate::zelda3::model::UWDoor;
use crate::zelda3::model::UWDoorDirection;
use crate::zelda3::model::UWDoorList;
use crate::zelda3::model::UWDoorPosition;
use crate::zelda3::model::UWDoorStyle;
use crate::zelda3::model::UWLayout;
use crate::zelda3::model::UWLayoutId;
use crate::zelda3::model::UWObject;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UWScene;

const STOP_MARKER: u8 = 0xFF;
const LAYER_MARKER: u8 = 0xFF;
const END_MARKER: u8 = 0xF0;

impl ReadObject<BTreeMap<UWRoomId, UWScene>> for SnesGame {
    fn read_objects(&self) -> BTreeMap<UWRoomId, UWScene> {
        BTreeMap::from_iter(UWRoomId::iter().map(|id| {
            let layout = read_layout(self, id);
            let doors = read_doors(self, id);
            (id, UWScene { layout, doors })
        }))
    }
}

/// Read an underworld layout
/// Note: The same layout may be referenced by multiple rooms. Any change will cause a new
/// record to be written instead of sharing the changes.
fn read_layout(game: &SnesGame, id: UWRoomId) -> UWLayout {
    let base_address = game.read_pointer_int24(Symbol::LayoutPtrs as usize + (id as usize * 3));
    let mut index = base_address;

    let preamble_bytes = game.read_all(base_address, 2);
    let (floor1, floor2, layout, aux0) = bytes_to_preamble(preamble_bytes);
    index += 2;

    let mut layers: Vec<Vec<UWObject>> = vec![vec![], vec![], vec![]];
    let mut layer_index = 0;
    loop {
        let bytes = game.read_all(index, 3);

        if bytes[0] == LAYER_MARKER && bytes[1] == LAYER_MARKER {
            layer_index += 1;
            index += 2;
            continue;
        }
        if bytes[0] == END_MARKER && bytes[1] == LAYER_MARKER {
            break;
        }

        layers[layer_index].push(bytes_to_object(bytes));

        // Move to the next value.
        index += 3;
    }

    UWLayout {
        floor1,
        floor2,
        layout,
        layers,
        aux0,
    }
}

fn read_doors(game: &SnesGame, id: UWRoomId) -> UWDoorList {
    let door_address = game.read_pointer_int24(Symbol::DoorPtrs as usize + (id as usize * 3));
    let mut index = door_address;
    let mut doors = vec![];
    loop {
        let bytes = game.read_all(index, 2);
        if bytes[0] == STOP_MARKER && bytes[1] == STOP_MARKER {
            break;
        }
        // The least significant bits represent the direction of the door.
        let direction = UWDoorDirection::from_repr(bytes[0] & 0b111).unwrap();
        let placement = UWDoorPosition::from_repr(bytes[0] >> 3).unwrap();
        let style = UWDoorStyle::from_repr(bytes[1]).unwrap();

        doors.push(UWDoor {
            style,
            direction,
            position: placement,
        });

        // Move to the next value.
        index += 2;
    }
    doors
}

/// This splits the preamble containing the floor types and layout into their
/// own integers.
fn bytes_to_preamble(bytes: &[u8]) -> (u8, u8, UWLayoutId, u8) {
    let byte0 = bytes[0];
    let floor1 = byte0 & 0b1111;
    let floor2 = byte0 >> 4;

    let byte1 = bytes[1];
    let layout = UWLayoutId::from_repr(byte1 >> 2).unwrap();
    // Preserve this for now until I can figure out what it is for.
    // In some cases this has a non-zero number. Maybe it has to do with collisions?
    let aux0 = byte1 & 0b11;

    (floor1, floor2, layout, aux0)
}

/// Converts a room object into a model representation. There are 3 subtypes.
fn bytes_to_object(bytes: &[u8]) -> UWObject {
    if bytes[2] >= 0xF8 {
        // Subtype 3
        let id: u16 = 0x200
            | ((bytes[2] as u16 - 0xF8) << 4)
            | ((bytes[1] as u16 & 0x3) << 2)
            | (bytes[0] as u16 & 0x3);
        let x: u8 = (bytes[0] & 0xFC) >> 2;
        let y: u8 = (bytes[1] & 0xFC) >> 2;
        let size: u8 = 0;
        return UWObject::from_value(id, x, y, size, size);
    }
    if bytes[0] >= 0xFC {
        // Subtype 2
        let id: u16 = (bytes[2] as u16 & 0x3F) | 0x100;
        let x: u8 = (bytes[1] >> 4) | ((bytes[0] & 0x3) << 4);
        let y: u8 = ((bytes[1] & 0xF) << 2) | bytes[2] >> 6;
        let size = 0;
        return UWObject::from_value(id, x, y, size, size);
    }

    // Subtype 1
    let id: u16 = bytes[2] as u16;
    let x: u8 = (bytes[0] & 0xFC) >> 2;
    let y: u8 = (bytes[1] & 0xFC) >> 2;
    // This is a guess that these two bits are the x size and 2 bits are the y size.
    // If bytes[0] contains the x position, it probably contains the x size in the last two bits.
    // Similarly, bytes[1] is the y position, so it logically follows that it contains the y size.
    let size_x: u8 = bytes[0] & 0x3;
    let size_y: u8 = bytes[1] & 0x3;
    UWObject::from_value(id, x, y, size_x, size_y)
}

#[cfg(test)]
mod tests {
    use assembly::zelda3::Symbol;
    use strum::IntoEnumIterator;

    use super::STOP_MARKER;
    use crate::snes::SnesGame;
    use crate::zelda3::io::uw_scene_reader::bytes_to_object;
    use crate::zelda3::io::uw_scene_reader::bytes_to_preamble;
    use crate::zelda3::io::uw_scene_reader::read_doors;
    use crate::zelda3::model::UWLayoutId;
    use crate::zelda3::model::UWObject;
    use crate::zelda3::model::UWRoomId;

    #[test]
    fn read_empty() {
        let game = init_with_empty_doors();
        let doors = read_doors(&game, UWRoomId::x00_GANON);
        assert_eq!(doors.len(), 0);
    }

    #[test]
    fn read_door_list() {
        let game = init_with_sample_doors();
        let room_without_doors = read_doors(&game, UWRoomId::x01_HYRULE_CASTLE_NORTH_CORRIDOR);
        let room_with_doors = read_doors(&game, UWRoomId::x00_GANON);

        assert_eq!(room_without_doors.len(), 0);
        assert_eq!(room_with_doors.len(), 12);
    }

    fn init_with_empty_doors() -> SnesGame {
        let mut game = SnesGame::new(&mut vec![0xFF; 0xFFFFF]);

        let mut ptr_cursor = Symbol::DoorPtrs as usize;
        let mut subroutine_cursor = 0x1F_8780;
        for _ in UWRoomId::iter() {
            // Add pointers to the empty position for each room.
            game.write_int24(ptr_cursor, subroutine_cursor);
            ptr_cursor += 3;

            // Allocate 24 doors.
            game.write_all(subroutine_cursor, &[STOP_MARKER, STOP_MARKER]);
            subroutine_cursor += (2 * 24) + 2
        }
        game
    }

    fn init_with_sample_doors() -> SnesGame {
        let mut game = init_with_empty_doors();
        let all_doors_bytes = get_sample_bytes();
        let cursor = 0x1F_8780;
        game.write_all(cursor, &all_doors_bytes);
        // Rewrite the door pointers for ganon.
        game.write_int24(Symbol::DoorPtrs as usize, cursor);
        game
    }

    #[rustfmt::skip]
    fn get_sample_bytes() -> Vec<u8> {
        vec![
            0x00, 0x00,
            0x10, 0x02,
            0x20, 0x04,
            0x30, 0x06,
            0x40, 0x08,
            0x50, 0x0A,
            0x60, 0x0C,
            0x70, 0x0E,
            0x80, 0x10,
            0x90, 0x12,
            0xA0, 0x14,
            0xB0, 0x16,
            STOP_MARKER, STOP_MARKER,
        ]
    }

    #[test]
    fn converts_bytes_to_preamble() {
        let actual = bytes_to_preamble(&[0x21, 0x1B]);
        let expected = (0x1, 0x2, UWLayoutId::X6_BottomSplit, 0x3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_bytes_to_object_subtype1() {
        let bytes = [0xC6, 0x17, 0xC4];
        let actual = bytes_to_object(&bytes);
        let expected = UWObject::from_value(0xC4, 0x31, 0x05, 2, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_bytes_to_object_subtype2() {
        let bytes = [0xFE, 0xFA, 0xA0];
        let actual = bytes_to_object(&bytes);
        let expected = UWObject::from_value(0x120, 0x2F, 0x2A, 0, 0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_bytes_to_object_subtype3() {
        let bytes = [0x83, 0x11, 0xFF];
        let actual = bytes_to_object(&bytes);
        let expected = UWObject::from_value(0x277, 0x20, 0x04, 0, 0);
        assert_eq!(actual, expected);
    }
}
