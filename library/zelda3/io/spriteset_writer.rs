use std::collections::BTreeMap;
use assembly::zelda3::Symbol;

use crate::common::readerwriter::WriteObject;
use crate::snes::SnesGame;
use crate::zelda3::model::Spriteset;
use crate::zelda3::model::SpritesetId;

impl WriteObject<BTreeMap<SpritesetId, Spriteset>> for SnesGame {
    fn write_objects(&mut self, spritesets: &BTreeMap<SpritesetId, Spriteset>) {
        for spriteset in spritesets.values() {
            self.write(
                Symbol::Spriteset as usize + (spriteset.id as usize * 4),
                spriteset.sheets[0] as u8,
            );
            self.write(
                Symbol::Spriteset as usize + (spriteset.id as usize * 4) + 1,
                spriteset.sheets[1] as u8,
            );
            self.write(
                Symbol::Spriteset as usize + (spriteset.id as usize * 4) + 2,
                spriteset.sheets[2] as u8,
            );
            self.write(
                Symbol::Spriteset as usize + (spriteset.id as usize * 4) + 3,
                spriteset.sheets[3] as u8,
            );
        }
    }
}
