use assembly::zelda3::Symbol;

use crate::common::readerwriter::WriteObject;
use crate::snes::SnesGame;
use crate::zelda3::model::DamageSubclass;

impl WriteObject<[DamageSubclass; 10]> for SnesGame {
    fn write_objects(&mut self, subclasses: &[DamageSubclass; 10]) {
        for (index, row) in subclasses.iter().enumerate() {
            self.write_all(
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
}
