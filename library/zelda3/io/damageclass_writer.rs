use assembly::zelda3::Symbol;
use common::SnesGame;

use crate::zelda3::model::DamageClass;

pub(super) fn write_damage_classes(game: &mut SnesGame, classes: &[DamageClass; 10]) {
    for (index, row) in classes.iter().enumerate() {
        // Write Link's damage table.
        game.write(Symbol::DamageClass as usize + (index * 3), row.green_mail);
        game.write(
            Symbol::DamageClass as usize + (index * 3) + 1,
            row.blue_mail,
        );
        game.write(Symbol::DamageClass as usize + (index * 3) + 2, row.red_mail);
    }
}
