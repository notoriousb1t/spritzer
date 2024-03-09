use assembly::zelda3::Symbol;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

use crate::common::readerwriter::ReadObject;
use crate::snes::bytes_to_int24;
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
            bytes_to_int24([header_bank, header_16bit_ptr[1], header_16bit_ptr[0]]);

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

    // Read in the graphics block which controls the spritesheets
    // and tags which declare behaviors.
    let bg2_property = data[0];
    let palette_id: PaletteId = PaletteId::from_repr(data[1]).expect(&format!(
        "UW Room {} palette load error {:02x}",
        id, data[1]
    ));
    let blockset_id = UWBlocksetId::from_repr(data[2]).expect(&format!(
        "UW Room {} spriteset load error {:02x}",
        id, data[2]
    ));

    let spriteset_id = SpritesetId::from_room_value(data[3]);
    let bgmove = data[4];
    let tag1 = UWRoomTag::from_repr(data[5])
        .expect(&format!("UW Room {} tag1 load error {:02x}", id, data[5]));
    let tag2 = UWRoomTag::from_repr(data[6])
        .expect(&format!("UW Room {} tag2 load error {:02x}", id, data[6]));
    let floor_upper = UWFloorId::from_repr(data[7]).expect(&format!(
        "UW Room {} floor upper load error {:02x}",
        id, data[7]
    ));

    let floor_lower = UWFloorId::from_repr(data[8]).expect(&format!(
        "UW Room {} floor lower load error {:02x}",
        id, data[8]
    ));
    let warp = UWRoomId::from_repr(data[9] as u16).expect(&format!(
        "UW Room {} warp/pit load error {:02x}",
        id, data[9]
    ));
    let stairs0 = UWRoomId::from_repr(data[10] as u16).expect(&format!(
        "UW Room {} stairs slot 0 load error {:02x}",
        id, data[10]
    ));
    let stairs1 = UWRoomId::from_repr(data[11] as u16).expect(&format!(
        "UW Room {} stairs slot 1 load error {:02x}",
        id, data[11]
    ));
    let stairs2 = UWRoomId::from_repr(data[12] as u16).expect(&format!(
        "UW Room {} stairs slot 2 load error {:02x}",
        id, data[12]
    ));
    let stairs3 = UWRoomId::from_repr(data[13] as u16).expect(&format!(
        "UW Room {} stairs slot 3 load error {:02x}",
        id, data[13]
    ));

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
