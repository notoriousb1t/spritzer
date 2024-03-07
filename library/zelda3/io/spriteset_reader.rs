use assembly::zelda3::Symbol;
use std::collections::BTreeMap;
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
    let bytes = game.read_all(Symbol::Spriteset as usize + (id as usize * 4), 4);

    Spriteset {
        id,
        sheets: [
            SpriteSheetId::from_repr(bytes[0]).expect(&format!(
                "Spriteset {} spritesheet ${:02X} id failure in slot 0",
                id, bytes[0]
            )),
            SpriteSheetId::from_repr(bytes[1]).expect(&format!(
                "Spriteset {} spritesheet ${:02X} id failure in slot 1",
                id, bytes[1]
            )),
            SpriteSheetId::from_repr(bytes[2]).expect(&format!(
                "Spriteset {} spritesheet ${:02X} id failure in slot 2",
                id, bytes[2]
            )),
            SpriteSheetId::from_repr(bytes[3]).expect(&format!(
                "Spriteset {} spritesheet ${:02X} id failure in slot 3",
                id, bytes[3]
            )),
        ],
    }
}
