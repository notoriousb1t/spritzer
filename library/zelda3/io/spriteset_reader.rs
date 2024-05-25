use std::collections::BTreeMap;

use common::SnesGame;
use strum::IntoEnumIterator;

use crate::zelda3::model::Spriteset;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::SpritesheetId;
use crate::zelda3::Addresses;

pub(super) fn read_spritesets(
    game: &SnesGame,
    addresses: &Addresses,
) -> BTreeMap<SpritesetId, Spriteset> {
    let mut values: Vec<(SpritesetId, Spriteset)> = vec![];
    for id in SpritesetId::iter() {
        values.push((id, _read_spriteset(game, addresses, id)));
    }
    BTreeMap::from_iter(values)
}

fn _read_spriteset(game: &SnesGame, addresses: &Addresses, id: SpritesetId) -> Spriteset {
    let bytes = game.read_all(addresses.spriteset + (id as usize * 4), 4);
    Spriteset {
        id,
        sheets: bytes_to_spritesheets(bytes),
    }
}

fn bytes_to_spritesheets(bytes: &[u8]) -> [SpritesheetId; 4] {
    [
        SpritesheetId::from_repr(bytes[0]).expect(&format!(
            "Spritesheet ${:02X} id failure in slot 0",
            bytes[0]
        )),
        SpritesheetId::from_repr(bytes[1]).expect(&format!(
            "Spritesheet ${:02X} id failure in slot 1",
            bytes[1]
        )),
        SpritesheetId::from_repr(bytes[2]).expect(&format!(
            "Spritesheet ${:02X} id failure in slot 2",
            bytes[2]
        )),
        SpritesheetId::from_repr(bytes[3]).expect(&format!(
            "Spritesheet ${:02X} id failure in slot 3",
            bytes[3]
        )),
    ]
}
