use common::SnesGame;

use crate::zelda3::model::UWRoomId;
use crate::zelda3::Addresses;

pub(super) fn write_pit_damage(
    game: &mut SnesGame,
    addresses: &Addresses,
    pit_damage: &[UWRoomId; 57],
) {
    let pit_damage_table = game.read_pointer_int24(addresses.pit_damage);
    let bytes = pit_damage
        .iter()
        .flat_map(|room_id| {
            let room_val = *room_id as u16;
            [
                (room_val & 0b1111_1111) as u8,
                ((room_val >> 8) & 0b1111_1111) as u8,
            ]
        })
        .collect::<Vec<_>>();
    game.write_all(pit_damage_table, &bytes);
}
