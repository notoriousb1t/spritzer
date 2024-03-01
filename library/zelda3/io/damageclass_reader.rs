use assembly::zelda3::Symbol;

use crate::common::readerwriter::WriteObject;
use crate::snes::SnesGame;
use crate::zelda3::model::DamageClass;

impl WriteObject<[DamageClass; 10]> for SnesGame {
    fn write_objects(&mut self, classes: &[DamageClass; 10]) {
        for (index, row) in classes.iter().enumerate() {
            // Write Link's damage table.
            self.write(Symbol::DamageClass as usize + (index * 3), row.green_mail);
            self.write(
                Symbol::DamageClass as usize + (index * 3) + 1,
                row.blue_mail,
            );
            self.write(Symbol::DamageClass as usize + (index * 3) + 2, row.red_mail);
        }
    }
}
