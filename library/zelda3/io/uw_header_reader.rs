use assembly::zelda3::Symbol;
use log::info;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

use crate::common::readerwriter::ReadObject;
use crate::snes::bytes_to_address;
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
        let header_16bit_ptr = self.read_all(Symbol::UWHeaderRef0.into(), 2);
        let header_bank = self.read(Symbol::UWHeaderBank.into());
        let header_pointer =
            bytes_to_address([header_bank, header_16bit_ptr[1], header_16bit_ptr[0]]);

        let mut values: Vec<(UWRoomId, UnderworldRoomHeader)> = vec![];
        for id in UWRoomId::iter() {
            values.push((id, _read_header(self, header_pointer, id)));
        }
        BTreeMap::from_iter(values)
    }
}

fn _read_header(game: &SnesGame, pointer_address: usize, id: UWRoomId) -> UnderworldRoomHeader {
    // Find the address of the Dungeon Room and read in graphics block and tags.
    let header_address = game.read_pointer_int16(pointer_address + (id as usize * 2));
    let data = game.read_all(header_address, 14);
    info!("Room {}; data {:?}", id, data);

    // Read in the graphics block which controls the spritesheets
    // and tags which declare behaviors.
    let bg2_property = data[0];
    let palette_id: PaletteId = PaletteId::from_repr(data[1]).unwrap();
    let blockset_id = UWBlocksetId::from_repr(data[2]).unwrap();

    let spriteset_id = SpritesetId::from_room_value(data[3]);
    let bgmove = data[4];
    let tag1 = UWRoomTag::from_repr(data[5]).unwrap();
    let tag2 = UWRoomTag::from_repr(data[6]).unwrap();
    let floor_upper = UWFloorId::from_repr(data[7]).unwrap();

    let floor_lower = UWFloorId::from_repr(data[8]).unwrap();
    let warp = UWRoomId::from_repr(data[9] as u16).unwrap();
    let stairs0 = UWRoomId::from_repr(data[10] as u16).unwrap();
    let stairs1 = UWRoomId::from_repr(data[11] as u16).unwrap();
    let stairs2 = UWRoomId::from_repr(data[12] as u16).unwrap();
    let stairs3 = UWRoomId::from_repr(data[13] as u16).unwrap();

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
