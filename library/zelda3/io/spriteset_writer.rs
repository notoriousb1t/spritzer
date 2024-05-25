use std::collections::BTreeMap;

use common::SnesGame;

use crate::zelda3::model::Spriteset;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::Addresses;

pub(super) fn write_spritesets(
    game: &mut SnesGame,
    addresses: &Addresses,
    spritesets: &BTreeMap<SpritesetId, Spriteset>,
) {
    for spriteset in spritesets.values() {
        game.write_all(
            addresses.spriteset + (spriteset.id as usize * 4),
            &spriteset_to_bytes(spriteset),
        );
    }
}

fn spriteset_to_bytes(spriteset: &Spriteset) -> [u8; 4] {
    [
        spriteset.sheets[0].into(),
        spriteset.sheets[1].into(),
        spriteset.sheets[2].into(),
        spriteset.sheets[3].into(),
    ]
}
