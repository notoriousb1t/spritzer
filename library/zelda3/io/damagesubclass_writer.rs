use common::SnesGame;

use crate::zelda3::model::DamageSubclass;
use crate::zelda3::options::Addresses;

pub(super) fn write_damage_subclasses(game: &mut SnesGame, addresses: &Addresses, subclasses: &[DamageSubclass; 10]) {
    for (index, row) in subclasses.iter().enumerate() {
        game.write_all(
            addresses.damage_subclass + (index * 8),
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
