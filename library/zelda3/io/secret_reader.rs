use std::collections::BTreeMap;

use common::SnesGame;
use strum::IntoEnumIterator;

use crate::zelda3::model::{HiddenItem, OWRoomId, OWSecrets, Secret, UWRoomId};
use crate::zelda3::Addresses;

pub(super) fn read_pot_secrets(game: &SnesGame, addresses: &Addresses) -> BTreeMap<UWRoomId, Vec<Secret>> {
    let table_pointer = game.read_pointer_int24(addresses.pot_secret_ptrs);

    let mut values: Vec<(UWRoomId, Vec<Secret>)> = vec![];
    for id in UWRoomId::iter() {
        let secrets_pointer = game.read_pointer_int16(table_pointer as usize + (id as usize * 2));
        values.push((id, bytes_to_secrets(game, secrets_pointer)));
    }
    BTreeMap::from_iter(values)
}

pub(super) fn read_bush_secrets(game: &SnesGame, addresses: &Addresses) -> BTreeMap<OWRoomId, OWSecrets> {
    let table_pointer = game.read_pointer_int24(addresses.bush_secret_ptrs);

    let mut values: Vec<(OWRoomId, OWSecrets)> = vec![];
    for id in OWRoomId::iter() {
        if id == OWRoomId::x40_MASTER_SWORD_UNDER_BRIDGE || id == OWRoomId::x41_ZORAS_DOMAIN {
            continue;
        }

        let lw_secrets_pointer = game.read_pointer_int16(table_pointer as usize + (id as usize * 2));
        let dw_secrets_pointer = game.read_pointer_int16(table_pointer as usize + ((id as usize + 0x40) * 2));
        let ow_secrets = OWSecrets {
            light_world: bytes_to_secrets(game, lw_secrets_pointer),
            dark_world: bytes_to_secrets(game, dw_secrets_pointer),
        };

        values.push((id, ow_secrets));
    }
    BTreeMap::from_iter(values)
}

// PotSecretPtrs
fn bytes_to_secrets(game: &SnesGame, secrets_pointer: usize) -> Vec<Secret> {
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
        let is_lower_layer = (position >> 15) != 0;

        // Read the item if it follows.
        let secret_byte = game.read(cursor + 2);
        let secret = HiddenItem::from_repr(secret_byte).expect("Invalid secret");
        secrets.push(Secret {
            x,
            y,
            is_lower_layer,
            item: Some(secret),
        });

        cursor += RECORD_SIZE;
    }

    secrets
}
