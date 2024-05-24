use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::vec;

use assembly::zelda3::Symbol;
use common::{int24_to_bytes, SnesGame};

use crate::zelda3::model::{PotSecret, UWRoomId};

pub(super) fn write_uw_pot_secrets(
    game: &mut SnesGame,
    pot_secrets: &BTreeMap<UWRoomId, Vec<PotSecret>>,
) {
    // It isn't great that this is hard coded in the function, but probably more
    // trouble to undo this.
    const BANK: u8 = 0x01;
    // There is an underlying assumption that omitting pointers for rooms 128-13F is safe,
    // it appears to be and that allows reclaiming some bytes that are effectively unused.
    game.mark(BANK, 0xDDB9, 0xE6B0);

    let mut header_pointer_map: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
    let mut pointers = vec![];
    for secrets in pot_secrets.values() {
        let bytes = secrets_to_bytes(&secrets);
        let pointer = match header_pointer_map.entry(bytes.to_owned()) {
            Entry::Vacant(it) => {
                let secrets_pointer = game
                    .write_data(&[BANK], &bytes)
                    .expect("Could not write pot secrets");
                *it.insert(secrets_pointer)
            }
            Entry::Occupied(val) => *val.get(),
        };
        pointers.push(pointer);
    }

    let table_pointer = game.read_pointer_int24(Symbol::PotSecretPtrs.into());
    // Write the pointers table.
    for (i, pointer) in pointers.iter().enumerate() {
        game.write_pointer_int16(table_pointer + (i * 2), *pointer);
    }
}

fn secrets_to_bytes(pot_secrets: &[PotSecret]) -> Vec<u8> {
    let mut bytes = vec![];

    for pot_secret in pot_secrets {
        if pot_secret.secret.is_none() {
            continue;
        }

        let position = ((if pot_secret.z { 1 } else { 0 }) << 15)
            | ((pot_secret.y as usize & 0b111111) << 7)
            | ((pot_secret.x as usize & 0b111111) << 1);
        let position_bytes = int24_to_bytes(position);
        bytes.push(position_bytes[2]);
        bytes.push(position_bytes[1]);
        bytes.push(pot_secret.secret.unwrap() as u8);
    }

    bytes.push(0xFF);
    bytes.push(0xFF);
    bytes
}
