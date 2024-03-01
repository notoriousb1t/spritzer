use assembly::zelda3::Symbol;

use crate::common::readerwriter::ReadObject;
use crate::snes::SnesGame;
use crate::zelda3::model::DamageSubclass;

impl ReadObject<[DamageSubclass; 10]> for SnesGame {
    /// SNES 0x36F33 - Damage ptr of weapons - 4 bytes (also determines which enemies you can kill
    /// with, and which enemies will be one-hit-ko)
    /// NOTE: this doesn't seem to work with Archipelago (probably intercepting damage subroutine).
    fn read_objects(&self) -> [DamageSubclass; 10] {
        let mut subclass_table: [DamageSubclass; 10] = [DamageSubclass::default(); 10];
        for index in 0..10 {
            let row = subclass_table.get_mut(index).unwrap();
            row.boomerang = self.read(Symbol::DamageSubclass as usize + (index * 8));
            row.sword1 = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0x1);
            row.sword2 = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0x2);
            row.sword3 = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0x3);
            row.sword4 = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0x4);
            row.sword5 = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0x5);
            row.arrow1 = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0x6);
            row.hookshot = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0x7);
            row.bomb = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0x8);
            row.arrow2 = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0x9);
            row.powder = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0xA);
            row.fire_rod = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0xB);
            row.ice_rod = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0xC);
            row.bombos = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0xD);
            row.ether = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0xE);
            row.quake = self.read(Symbol::DamageSubclass as usize + (index * 8) + 0xF);
        }
        subclass_table
    }
}
