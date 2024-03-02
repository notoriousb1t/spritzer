use std::collections::BTreeMap;
use assembly::zelda3::Symbol;
use strum::IntoEnumIterator;

use crate::common::readerwriter::ReadObject;
use crate::snes::SnesGame;
use crate::zelda3::model::SpriteSheetId;
use crate::zelda3::model::Spriteset;
use crate::zelda3::model::SpritesetId;

impl ReadObject<BTreeMap<SpritesetId, Spriteset>> for SnesGame {
    fn read_objects(&self) -> BTreeMap<SpritesetId, Spriteset> {
        let mut values: Vec<(SpritesetId, Spriteset)> = vec![];
        for id in SpritesetId::iter() {
            values.push((id, _read_spriteset(self, id)));
        }
        BTreeMap::from_iter(values)
    }
}

fn _read_spriteset(game: &SnesGame, id: SpritesetId) -> Spriteset {
    let sheet0 = game.read(Symbol::Spriteset as usize + (id as usize * 4));
    let sheet1 = game.read(Symbol::Spriteset as usize + (id as usize * 4) + 1);
    let sheet2 = game.read(Symbol::Spriteset as usize + (id as usize * 4) + 2);
    let sheet3 = game.read(Symbol::Spriteset as usize + (id as usize * 4) + 3);
    Spriteset {
        id,
        sheets: [
            SpriteSheetId::from_repr(sheet0).unwrap(),
            SpriteSheetId::from_repr(sheet1).unwrap(),
            SpriteSheetId::from_repr(sheet2).unwrap(),
            SpriteSheetId::from_repr(sheet3).unwrap(),
        ],
    }
}
