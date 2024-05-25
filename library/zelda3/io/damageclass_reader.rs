use common::SnesGame;

use crate::zelda3::model::DamageClass;
use crate::zelda3::options::Addresses;

pub(super) fn read_damage_classes(game: &SnesGame, addresses: &Addresses) -> [DamageClass; 10] {
    let mut damage_table: [DamageClass; 10] = [DamageClass::default(); 10];
    for index in 0..10 {
        // Read Link's damage table.
        let row = damage_table.get_mut(index).unwrap();
        row.green_mail = game.read(addresses.damage_class + (index * 3));
        row.blue_mail = game.read(addresses.damage_class + (index * 3) + 1);
        row.red_mail = game.read(addresses.damage_class + (index * 3) + 2);
    }
    damage_table
}
