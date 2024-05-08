use log;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::zelda3::model::{
    get_sprite_type, get_weights, is_fully_compatible, is_partially_compatible, OWRoomId,
    OWRoomState, Placement, SpriteId, SpriteType, Z3Model,
};
use crate::zelda3::Balancing;

pub(crate) fn apply_ow_sprites_chaotic_shuffle(model: &mut Z3Model) {
    model.prepare_sprite_pool();

    let mut rng = model.create_rng();
    for room_id in OWRoomId::iter() {
        let room = model.ow_rooms.get_mut(&room_id).unwrap();
        for state in room.versions_mut() {
            let choices = model.sprite_pool[&state.spriteset_id].to_owned();
            randomize_sprites(model.uw_balancing, choices, &mut rng, room_id, state);
        }
    }
}

fn randomize_sprites(
    balancing: Balancing,
    choices: Vec<SpriteId>,
    rng: &mut StdRng,
    room_id: OWRoomId,
    state: &mut OWRoomState,
) {
    // Loop through all sprites, marking sprites as needing removal.
    let mut removed_indexes: Vec<usize> = vec![];
    for (index, uw_sprite) in state.sprites.iter_mut().enumerate() {
        // Find the subset of possible choices.
        let mut possible_matches: Vec<&SpriteId> = choices
            .iter()
            .filter(|&it| is_fully_compatible(&uw_sprite.id, it, Placement::Area, false))
            .collect::<Vec<_>>();

        if possible_matches.is_empty() {
            // If there is nothing that is fully compatible, use looser rules
            possible_matches = choices
                .iter()
                .filter(|&it| is_partially_compatible(&uw_sprite.id, it, Placement::Area, false))
                .collect::<Vec<_>>();
            if possible_matches.is_empty() {
                match get_sprite_type(&uw_sprite.id) {
                    SpriteType::Object => {}
                    SpriteType::Npc => {}
                    _ => {
                        // Mark sprite as removed.
                        removed_indexes.push(index);
                    }
                }
                continue;
            }
        }

        // Compute the weights.
        let weights = get_weights(&balancing, false, &possible_matches);
        if weights.iter().all(|it| it.1 == &0) {
            // Mark sprite as irreplaceable.
            continue;
        }

        let sprite_result = possible_matches
            .choose_weighted(rng, |&it| weights[it])
            .unwrap();

        log::debug!(
            "UW ${:02X} -- {} -> {}",
            room_id as u8,
            uw_sprite.id,
            sprite_result
        );

        uw_sprite.id = **sprite_result;
    }

    // Remove any sprites that couldn't be replaced.
    while let Some(pos) = removed_indexes.pop() {
        state.sprites.remove(pos);
    }
}
