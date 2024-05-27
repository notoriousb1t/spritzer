use common::SnesGame;

use crate::zelda3::model::UWRoomId;
use crate::zelda3::Addresses;

pub(super) fn read_pit_damage(game: &SnesGame, addresses: &Addresses) -> [UWRoomId; 57] {
    let pit_damage_table = game.read_pointer_int24(addresses.pit_damage);
    let data = game.read_all::<114>(pit_damage_table);
    let mut result = [UWRoomId::x05_EMPTY_CLONE_ROOM1; 57];
    for i in 0..57 {
        result[i] = UWRoomId::from_repr(((data[(i * 2) + 1] as u16) << 8) | data[i * 2] as u16)
            .expect("Invalid room in pit damage");
    }
    result
}
