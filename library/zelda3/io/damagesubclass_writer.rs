use common::SnesGame;

use crate::zelda3::model::DamageSubclass;
use crate::zelda3::options::Addresses;

pub(super) fn write_damage_subclasses(
    game: &mut SnesGame,
    addresses: &Addresses,
    subclasses: &[DamageSubclass; 8],
) {
    let bytes = damage_subclass_table_to_bytes(subclasses);
    game.write_all(addresses.damage_subclass, &bytes);
}

fn damage_subclass_table_to_bytes(subclasses: &[DamageSubclass; 8]) -> [u8; 128] {
    let mut bytes: [u8; 128] = [0; 128];
    for i in 0..8 {
        bytes[(i * 8) + 0] = subclasses[i].boomerang;
        bytes[(i * 8) + 1] = subclasses[i].sword1;
        bytes[(i * 8) + 2] = subclasses[i].sword2;
        bytes[(i * 8) + 3] = subclasses[i].sword3;
        bytes[(i * 8) + 4] = subclasses[i].sword4;
        bytes[(i * 8) + 5] = subclasses[i].sword5;
        bytes[(i * 8) + 6] = subclasses[i].arrow1;
        bytes[(i * 8) + 7] = subclasses[i].hookshot;
        bytes[(i * 8) + 8] = subclasses[i].bomb;
        bytes[(i * 8) + 9] = subclasses[i].arrow2;
        bytes[(i * 8) + 10] = subclasses[i].powder;
        bytes[(i * 8) + 11] = subclasses[i].fire_rod;
        bytes[(i * 8) + 12] = subclasses[i].ice_rod;
        bytes[(i * 8) + 13] = subclasses[i].bombos;
        bytes[(i * 8) + 14] = subclasses[i].ether;
        bytes[(i * 8) + 15] = subclasses[i].quake;
    }
    bytes
}
