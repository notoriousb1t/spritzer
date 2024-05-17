use std::collections::BTreeMap;

use assembly::zelda3::Symbol;
use common::SnesGame;
use strum::IntoEnumIterator;

use crate::zelda3::model::Spriteset;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::SpritesheetId;

pub(super) fn read_spritesets(game: &SnesGame) -> BTreeMap<SpritesetId, Spriteset> {
    let mut values: Vec<(SpritesetId, Spriteset)> = vec![];
    for id in SpritesetId::iter() {
        values.push((id, _read_spriteset(game, id)));
    }
    BTreeMap::from_iter(values)
}

fn _read_spriteset(game: &SnesGame, id: SpritesetId) -> Spriteset {
    let bytes = game.read_all(Symbol::Spriteset as usize + (id as usize * 4), 4);

    Spriteset {
        id,
        sheets: [
            SpritesheetId::from_repr(bytes[0]).expect(&format!(
                "Spriteset {} spritesheet ${:02X} id failure in slot 0",
                id, bytes[0]
            )),
            SpritesheetId::from_repr(bytes[1]).expect(&format!(
                "Spriteset {} spritesheet ${:02X} id failure in slot 1",
                id, bytes[1]
            )),
            SpritesheetId::from_repr(bytes[2]).expect(&format!(
                "Spriteset {} spritesheet ${:02X} id failure in slot 2",
                id, bytes[2]
            )),
            SpritesheetId::from_repr(bytes[3]).expect(&format!(
                "Spriteset {} spritesheet ${:02X} id failure in slot 3",
                id, bytes[3]
            )),
        ],
    }
}
