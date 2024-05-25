use assembly::zelda3::Symbol;
use common::Patch;
use rand::seq::SliceRandom;

use crate::zelda3::model::Z3Model;
use crate::zelda3::options::DEBUG_MOLDORM_1;
use crate::zelda3::options::DEBUG_MOLDORM_2;
use crate::zelda3::options::DEBUG_MOLDORM_3;
use crate::zelda3::options::DEBUG_MOLDORM_4;
use crate::zelda3::options::DEBUG_MOLDORM_5;
use crate::zelda3::options::DEBUG_MOLDORM_6;
use crate::zelda3::options::DEBUG_MOLDORM_7;
use crate::zelda3::options::DEBUG_MOLDORM_8;

pub(crate) fn apply_moldorm_eye_shuffle(model: &mut Z3Model) {
    let mut rng = model.create_rng();
    let eye_count = match check_for_debug_string(&model.debug_string) {
        Some(count) => count,
        None => {
            let eye_count = *Vec::from_iter(1..9u8).choose(&mut rng).unwrap();
            eye_count
        }
    };

    // There is always at least one eye since the loop is a 0 based decrementing loop.
    model
        .patches
        .push(Patch::of(Symbol::MoldormEyeCount.into(), eye_count - 1));
}

fn check_for_debug_string(seed: &str) -> Option<u8> {
    for (word, eye_count) in [
        (DEBUG_MOLDORM_1, 1u8),
        (DEBUG_MOLDORM_2, 2u8),
        (DEBUG_MOLDORM_3, 3u8),
        (DEBUG_MOLDORM_4, 4u8),
        (DEBUG_MOLDORM_5, 5u8),
        (DEBUG_MOLDORM_6, 6u8),
        (DEBUG_MOLDORM_7, 7u8),
        (DEBUG_MOLDORM_8, 8u8),
    ] {
        if seed.contains(word) {
            return Some(eye_count);
        }
    }
    None
}
