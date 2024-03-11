//! Handles shuffling overlords in dungeons. Consider reworking to use assembly directly and flag.

use common::Patch;
use common::BEQ;
use common::NEGATIVE_MASK;
use rand::seq::SliceRandom;

use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::get_sprite_vulnerability;
use crate::zelda3::model::get_weights;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::SpriteVulnerability;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::Z3Model;

pub(crate) fn apply_uw_overlord_shuffle(model: &mut Z3Model) {
    reroll_eastern_palace_overlords(model);
}

fn reroll_eastern_palace_overlords(model: &mut Z3Model) {
    reroll_stalfos_appear(model);
    reroll_cannon_lobby(model);
}

/// It can't be assumed that stalfos are loaded, so load something else.
fn reroll_stalfos_appear(model: &mut Z3Model) {
    let mut rng = model.create_rng();

    if let Some(room) = model
        .uw_headers
        .get_mut(&UWRoomId::xAA_EASTERN_PALACE_MAP_CHEST_ROOM)
    {
        let choices: &Vec<&SpriteId> = &model.sprite_pool[&room.spriteset_id]
            .iter()
            .filter(|it| match get_sprite_type(it) {
                SpriteType::Hazard => false,
                SpriteType::Enemy => matches!(
                    get_sprite_vulnerability(it),
                    SpriteVulnerability::All | SpriteVulnerability::Invulnerable
                ),
                _ => false,
            })
            .collect();
        let weights = get_weights(&model.uw_balancing, choices);
        if let Ok(&sprite_id) = choices.choose_weighted(&mut rng, |sprite_id| weights[*sprite_id]) {
            model.patches.push(Patch::of(0x9B832, *sprite_id as u8));
        }
    }
}

fn reroll_cannon_lobby(model: &mut Z3Model) {
    let mut rng = model.create_rng();

    let eastern_palace_overlords = vec![noop, replace_cannon_overlord_with_big_cannons];

    if let Some(overlord_random) = eastern_palace_overlords.choose(&mut rng) {
        overlord_random(model);
    }
}

fn replace_cannon_overlord_with_big_cannons(model: &mut Z3Model) {
    // Increase the time between balls. This allows more time to go
    // between the larger balls. Without this modification, it is impossible to
    // avoid damage without cape, cane, etc. This value is normally 0x38.
    model.patches.push(Patch::of(0x9BF83, 0x80));
    // This changes the condition so the smaller ball is not chosen.
    model.patches.push(Patch::of(0x9BF8A, BEQ));
    // Increase cannonball speed. this also affects the 4-way canon room.
    let speed = 36;
    model
        .patches
        .push(Patch::from(0x9C016, &[speed, speed | NEGATIVE_MASK, 0, 0]));
    model
        .patches
        .push(Patch::from(0x9C01A, &[0, 0, speed, speed | NEGATIVE_MASK]));
}

fn noop(_model: &mut Z3Model) {}
