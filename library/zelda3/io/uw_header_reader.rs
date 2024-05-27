use std::collections::BTreeMap;

use common::bytes_to_int24;
use common::SnesGame;
use strum::IntoEnumIterator;

use crate::zelda3::model::PaletteId;
use crate::zelda3::model::RoomBackground;
use crate::zelda3::model::RoomCollision;
use crate::zelda3::model::RoomEffect;
use crate::zelda3::model::RoomLogic;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::UWBlocksetId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UnderworldRoomHeader;
use crate::zelda3::Addresses;

pub(super) fn read_uw_headers(
    game: &SnesGame,
    addresses: &Addresses,
) -> BTreeMap<UWRoomId, UnderworldRoomHeader> {
    let header_16bit_ptr = game.read_all::<2>(addresses.uwheader_ref0);
    let header_bank = game.read(addresses.uwheader_bank);
    let header_pointer = bytes_to_int24([header_bank, header_16bit_ptr[1], header_16bit_ptr[0]]);

    let mut values: Vec<(UWRoomId, UnderworldRoomHeader)> = vec![];
    for id in UWRoomId::iter() {
        // Find the address of the Dungeon Room and read in graphics block and tags.
        let header_address = game.read_pointer_int16(header_pointer + (id as usize * 2));
        let data = game.read_all::<14>(header_address);

        values.push((id, bytes_to_room_header(data, id)));
    }
    BTreeMap::from_iter(values)
}

fn bytes_to_room_header(data: [u8; 14], id: UWRoomId) -> UnderworldRoomHeader {
    // Read in the graphics block which controls the spritesheets
    // and tags which declare behaviors.
    let bg2 = RoomBackground::from_repr(data[0] >> 5).expect(&format!(
        "UW ${:02X} invalid room background ${:02X}",
        id as u16, data[0] >> 5
    ));
    let collision =
        RoomCollision::from_repr((data[0] & 0b11100) >> 2).expect("Invalid room collision");
    let light = (data[0] & 0b1) == 0b1;

    let palette_id: PaletteId = PaletteId::from_repr(data[1]).expect(&format!(
        "UW ${:02X} palette load error ${:02X}",
        id as u16, data[1]
    ));
    let blockset_id = UWBlocksetId::from_repr(data[2]).expect(&format!(
        "UW ${:02X} spriteset load error ${:02X}",
        id as u16, data[2]
    ));

    let spriteset_id = SpritesetId::from_room_value(data[3]);
    let effect = RoomEffect::from_repr(data[4]).expect("Invalid room effect");
    let tag1 = RoomLogic::from_repr(data[5]).expect(&format!(
        "UW ${:02X} tag1 load error ${:02X}",
        id as u16, data[5]
    ));
    let tag2 = RoomLogic::from_repr(data[6]).expect(&format!(
        "UW ${:02X} tag2 load error ${:02X}",
        id as u16, data[6]
    ));

    let holewarp_plane = data[7] & 0b11;
    let stairs1_plane = (data[7] >> 2) & 0b11;
    let stairs2_plane = (data[7] >> 4) & 0b11;
    let stairs3_plane = (data[7] >> 6) & 0b11;
    let stairs4_plane = data[8] & 0b11;

    let warp = UWRoomId::from_repr(data[9] as u16).expect(&format!(
        "UW ${:02X} warp/pit load error ${:02X}",
        id as u16, data[9]
    ));
    let stairs0 = UWRoomId::from_repr(data[10] as u16).expect(&format!(
        "UW ${:02X} stairs slot 0 load error ${:02X}",
        id as u16, data[10]
    ));
    let stairs1 = UWRoomId::from_repr(data[11] as u16).expect(&format!(
        "UW ${:02X} stairs slot 1 load error ${:02X}",
        id as u16, data[11]
    ));
    let stairs2 = UWRoomId::from_repr(data[12] as u16).expect(&format!(
        "UW ${:02X} stairs slot 2 load error ${:02X}",
        id as u16, data[12]
    ));
    let stairs3 = UWRoomId::from_repr(data[13] as u16).expect(&format!(
        "UW ${:02X} stairs slot 3 load error ${:02X}",
        id as u16, data[13]
    ));

    UnderworldRoomHeader {
        bg2,
        collision,
        light,
        palette_id,
        blockset_id,
        spriteset_id,
        effect,
        tag1,
        tag2,
        holewarp_plane,
        stairs1_plane,
        stairs2_plane,
        stairs3_plane,
        stairs4_plane,
        holewarp: warp,
        stairs1: stairs0,
        stairs2: stairs1,
        stairs3: stairs2,
        stairs4: stairs3,
    }
}
