use assembly::zelda3::Symbol;
use common::SnesGame;

use crate::zelda3::model::DamageSubclass;

pub(super) fn write_damage_subclasses(game: &mut SnesGame, subclasses: &[DamageSubclass; 10]) {
    for (index, row) in subclasses.iter().enumerate() {
        game.write_all(
            Symbol::DamageSubclass as usize + (index * 8),
            &[
                row.boomerang,
                row.sword1,
                row.sword2,
                row.sword3,
                row.sword4,
                row.sword5,
                row.arrow1,
                row.hookshot,
                row.bomb,
                row.arrow2,
                row.powder,
                row.fire_rod,
                row.ice_rod,
                row.bombos,
                row.ether,
                row.quake,
            ],
        );
    }
}
