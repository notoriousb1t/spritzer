use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::vec;

use common::{int24_to_bytes, SnesGame};

use crate::zelda3::model::{OWRoomId, OWSecrets, Secret, UWRoomId};
use crate::zelda3::Addresses;

pub(super) fn write_pot_secrets(
    game: &mut SnesGame, 
    addresses: &Addresses,
    pot_secrets: &BTreeMap<UWRoomId, Vec<Secret>>,
) {
    // It isn't great that this is hard coded in the function, but probably more
    // trouble not to hardcode it.
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
                    .write_data(BANK, &bytes)
                    .expect("Could not write pot secrets");
                *it.insert(secrets_pointer)
            }
            Entry::Occupied(val) => *val.get(),
        };
        pointers.push(pointer);
    }

    let table_pointer = game.read_pointer_int24(addresses.pot_secret_ptrs);
    // Write the pointers table.
    for (i, pointer) in pointers.iter().enumerate() {
        game.write_pointer_int16(table_pointer + (i * 2), *pointer);
    }
}

pub(super) fn write_bush_secrets(
    game: &mut SnesGame,
    addresses: &Addresses,
    secrets_map: &BTreeMap<OWRoomId, OWSecrets>,
) {
    // Mark the area as writable.
    const BANK: u8 = 0x1B;
    game.mark(BANK, 0xC3F9, 0xC89A);

    let mut header_pointer_map: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
    let mut lw_pointers: Vec<(OWRoomId, usize)> = vec![];
    let mut dw_pointers: Vec<(OWRoomId, usize)> = vec![];
    for (room_id, ow_secrets) in secrets_map.iter() {
        for (secrets, is_dark_world) in [(&ow_secrets.light_world, false), (&ow_secrets.dark_world, true)] {
            let bytes = secrets_to_bytes(&secrets);
            let pointer = match header_pointer_map.entry(bytes.to_owned()) {
                Entry::Vacant(it) => {
                    let secrets_pointer = game
                        .write_data(BANK, &bytes)
                        .expect("Could not write bush secrets");
                    *it.insert(secrets_pointer)
                }
                Entry::Occupied(val) => *val.get(),
            };
            if is_dark_world {
                dw_pointers.push((*room_id, pointer));
            } else {
                lw_pointers.push((*room_id, pointer));
            }
        }
    }

    let table_pointer = game.read_pointer_int24(addresses.bush_secret_ptrs);
    // Write the pointers table.
    for (room_id, pointer) in lw_pointers.iter() {
        game.write_pointer_int16(table_pointer + (*room_id as usize * 2), *pointer);
    }
    for (room_id, pointer) in dw_pointers.iter() {
        game.write_pointer_int16(table_pointer + ((*room_id as usize + 0x40) * 2), *pointer);
    }
}

fn secrets_to_bytes(pot_secrets: &[Secret]) -> Vec<u8> {
    let mut bytes = vec![];

    for pot_secret in pot_secrets {
        if pot_secret.item.is_none() {
            continue;
        }

        let position = ((if pot_secret.is_lower_layer { 1 } else { 0 }) << 15)
            | ((pot_secret.y as usize & 0b111111) << 7)
            | ((pot_secret.x as usize & 0b111111) << 1);
        let position_bytes = int24_to_bytes(position);
        bytes.push(position_bytes[2]);
        bytes.push(position_bytes[1]);
        bytes.push(pot_secret.item.unwrap() as u8);
    }

    bytes.push(0xFF);
    bytes.push(0xFF);
    bytes
}
