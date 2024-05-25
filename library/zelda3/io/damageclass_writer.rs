use common::SnesGame;

use crate::zelda3::options::Addresses;
use crate::zelda3::model::DamageClass;

pub(super) fn write_damage_classes(game: &mut SnesGame, addresses: &Addresses, classes: &[DamageClass; 10]) {
    for (index, row) in classes.iter().enumerate() {
        // Write Link's damage table.
        game.write(addresses.damage_class + (index * 3), row.green_mail);
        game.write(
            addresses.damage_class + (index * 3) + 1,
            row.blue_mail,
        );
        game.write(addresses.damage_class + (index * 3) + 2, row.red_mail);
    }
}
