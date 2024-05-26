use common::SnesGame;

use crate::zelda3::options::Addresses;
use crate::zelda3::model::DamageClass;

pub(super) fn write_damage_classes(game: &mut SnesGame, addresses: &Addresses, classes: &[DamageClass; 10]) {
    game.write_all(addresses.damage_class, &damage_table_to_bytes(classes));
}

fn damage_table_to_bytes(damage_classes: &[DamageClass; 10]) -> Vec<u8> {
    damage_classes.iter().flat_map(damage_class_to_bytes).collect::<Vec<_>>()
}

fn damage_class_to_bytes(damage_class: &DamageClass) -> [u8; 3] {
    [
        damage_class.green_mail,
        damage_class.blue_mail,
        damage_class.red_mail,
    ]
}