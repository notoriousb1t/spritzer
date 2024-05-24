use std::collections::BTreeMap;

use assembly::zelda3::Symbol;
use common::SnesGame;
use strum::IntoEnumIterator;

use crate::zelda3::model::{PotSecret, Secret, UWRoomId};

pub(super) fn read_uw_pot_secrets(game: &SnesGame) -> BTreeMap<UWRoomId, Vec<PotSecret>> {
    let table_pointer = game.read_pointer_int24(Symbol::PotSecretPtrs.into());

    let mut values: Vec<(UWRoomId, Vec<PotSecret>)> = vec![];
    for id in UWRoomId::iter() {
        let secrets_pointer = game.read_pointer_int16(table_pointer as usize + (id as usize * 2));
        values.push((id, read_pot_secrets(game, secrets_pointer)));
    }
    BTreeMap::from_iter(values)
}

// PotSecretPtrs
fn read_pot_secrets(game: &SnesGame, secrets_pointer: usize) -> Vec<PotSecret> {
    const RECORD_SIZE: usize = 3;
    const STOP_MARKER: u16 = 0xFFFF;

    let mut secrets = vec![];
    let mut cursor = secrets_pointer;
    loop {
        let position = game.read_int16(cursor);
        if position == STOP_MARKER {
            break;
        }

        // Best guess at serialization is z--yyyyy yxxxxxx-
        let x = ((position >> 1) & 0b111111) as u8;
        let y = ((position >> 7) & 0b111111) as u8;
        let z = (position >> 15) != 0;

        // Read the item if it follows.
        let secret_byte = game.read(cursor + 2);
        let secret = Secret::from_repr(secret_byte).expect("Bad pot item");

        secrets.push(PotSecret {
            x,
            y,
            z,
            secret: Some(secret),
        });

        cursor += RECORD_SIZE;
    }

    secrets
}
