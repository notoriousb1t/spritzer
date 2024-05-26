use common::SnesGame;

use crate::zelda3::model::DamageClass;
use crate::zelda3::options::Addresses;

pub(super) fn read_damage_classes(game: &SnesGame, addresses: &Addresses) -> [DamageClass; 10] {
    let bytes = game.read_all::<30>(addresses.damage_class);
    bytes_to_damage_table(&bytes)
}

fn bytes_to_damage_table(bytes: &[u8; 30]) -> [DamageClass; 10] {
    let mut damage_table: [DamageClass; 10] = [DamageClass::default(); 10];
    for (index, chunk) in bytes.chunks(3).enumerate() {
        damage_table[index] = bytes_to_damage_class(chunk.try_into().unwrap());
    }
    damage_table
}

fn bytes_to_damage_class(bytes: &[u8; 3]) -> DamageClass {
    DamageClass {
        green_mail: bytes[0],
        blue_mail: bytes[1],
        red_mail: bytes[2],
    }
}