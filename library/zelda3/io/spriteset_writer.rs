use assembly::zelda3::Symbol;
use std::collections::BTreeMap;

use crate::zelda3::model::Spriteset;
use crate::zelda3::model::SpritesetId;
use common::SnesGame;

pub(super) fn write_spritesets(game: &mut SnesGame, spritesets: &BTreeMap<SpritesetId, Spriteset>) {
    for spriteset in spritesets.values() {
        game.write(
            Symbol::Spriteset as usize + (spriteset.id as usize * 4),
            spriteset.sheets[0] as u8,
        );
        game.write(
            Symbol::Spriteset as usize + (spriteset.id as usize * 4) + 1,
            spriteset.sheets[1] as u8,
        );
        game.write(
            Symbol::Spriteset as usize + (spriteset.id as usize * 4) + 2,
            spriteset.sheets[2] as u8,
        );
        game.write(
            Symbol::Spriteset as usize + (spriteset.id as usize * 4) + 3,
            spriteset.sheets[3] as u8,
        );
    }
}
