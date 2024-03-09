use assembly::zelda3::Symbol;

use crate::zelda3::model::DamageClass;
use common::SnesGame;

pub(super) fn read_damage_classes(game: &SnesGame) -> [DamageClass; 10] {
    let mut damage_table: [DamageClass; 10] = [DamageClass::default(); 10];
    for index in 0..10 {
        // Read Link's damage table.
        let row = damage_table.get_mut(index).unwrap();
        row.green_mail = game.read(Symbol::DamageClass as usize + (index * 3));
        row.blue_mail = game.read(Symbol::DamageClass as usize + (index * 3) + 1);
        row.red_mail = game.read(Symbol::DamageClass as usize + (index * 3) + 2);
    }
    damage_table
}
