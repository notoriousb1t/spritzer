use std::collections::BTreeMap;
use assembly::zelda3::Symbol;
use strum::IntoEnumIterator;

use crate::common::readerwriter::ReadObject;
use crate::snes::SnesGame;
use crate::zelda3::model::PaletteId;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::UWBlocksetId;
use crate::zelda3::model::UWFloorId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UWRoomTag;
use crate::zelda3::model::UnderworldRoomHeader;

impl ReadObject<BTreeMap<UWRoomId, UnderworldRoomHeader>> for SnesGame {
    fn read_objects(&self) -> BTreeMap<UWRoomId, UnderworldRoomHeader> {
        let mut values: Vec<(UWRoomId, UnderworldRoomHeader)> = vec![];
        for id in UWRoomId::iter() {
            values.push((id, _read_header(self, id)));
        }
        BTreeMap::from_iter(values)
    }
}

fn _read_header(game: &SnesGame, id: UWRoomId) -> UnderworldRoomHeader {
    // Find the address of the Dungeon Room and read in graphics block and tags.
    let header_address = game.read_pointer_int16(Symbol::UwHeaderPtrs as usize + (id as usize * 2));
    // Read in the graphics block which controls the spritesheets
    // and tags which declare behaviors.
    let bg2_property = game.read(header_address);
    let palette_id = PaletteId::from_repr(game.read(header_address + 1)).unwrap();
    let blockset_id = UWBlocksetId::from_repr(game.read(header_address + 2)).unwrap();

    let spriteset_id = SpritesetId::from_room_value(game.read(header_address + 3));
    let bgmove = game.read(header_address + 4);
    let tag1 = UWRoomTag::from_repr(game.read(header_address + 5)).unwrap();
    let tag2 = UWRoomTag::from_repr(game.read(header_address + 6)).unwrap();
    let floor_upper = UWFloorId::from_repr(game.read(header_address + 7)).unwrap();

    let floor_lower = UWFloorId::from_repr(game.read(header_address + 8)).unwrap();
    let warp = UWRoomId::from_repr(game.read(header_address + 9) as u16).unwrap();
    let stairs0 = UWRoomId::from_repr(game.read(header_address + 10) as u16).unwrap();
    let stairs1 = UWRoomId::from_repr(game.read(header_address + 11) as u16).unwrap();
    let stairs2 = UWRoomId::from_repr(game.read(header_address + 12) as u16).unwrap();
    let stairs3 = UWRoomId::from_repr(game.read(header_address + 13) as u16).unwrap();

    UnderworldRoomHeader {
        id,
        bg2_property,
        palette_id,
        blockset_id,
        spriteset_id,
        bgmove,
        tag1,
        tag2,
        planes1: floor_upper,
        planes2: floor_lower,
        warp,
        stairs0,
        stairs1,
        stairs2,
        stairs3,
    }
}
