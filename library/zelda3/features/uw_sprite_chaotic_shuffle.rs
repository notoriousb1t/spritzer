use log;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::zelda3::model::{
    get_sprite_type, get_weights, is_fully_compatible, is_partially_compatible, DungeonId,
    Placement, SpriteId, SpriteType, UWRoomId, Z3Model,
};

pub(crate) fn apply_uw_sprites_chaotic_shuffle(model: &mut Z3Model) {
    let mut rng = model.create_rng();
    for room_id in UWRoomId::iter() {
        randomize_sprites(model, &mut rng, room_id);
    }
}

fn randomize_sprites(model: &mut Z3Model, rng: &mut StdRng, room_id: UWRoomId) {
    // Get the list of choices.
    let header = model
        .uw_headers
        .get(&room_id)
        .expect("UWRoomHeader To exist");

    // Get the placement strategy.
    let placement = match header.tag1.is_kill_room() || header.tag2.is_kill_room() {
        true => Placement::KillRoom,
        false => Placement::Room,
    };
    // Get the current sprite choices for the pool.
    let choices = &model.sprite_pool[&header.spriteset_id];
    // Get the current sprites in the room.
    let spritelist = model
        .uw_sprites
        .get_mut(&room_id)
        .expect("Sprites should exist");

    if spritelist
        .sprites
        .iter()
        .any(|sprite| get_sprite_type(&sprite.id) == SpriteType::Boss)
    {
        // Ignore boss rooms. Those are handled separately.
        return;
    }

    // If the room is part of escape, artificially lower the difficulty because this could be a standard
    // run and there are some rooms that are painful given the number of pests normally there.
    let is_adjusted_down = model
        .dungeons
        .iter()
        .find(|(dungeon_id, dungeon)| {
            **dungeon_id == DungeonId::X00_Sewers && dungeon.rooms.contains(&room_id)
        })
        .is_some();

    // Loop through all sprites, marking sprites as needing removal.
    let mut removed_indexes: Vec<usize> = vec![];
    for (index, uw_sprite) in spritelist.sprites.iter_mut().enumerate() {
        let sprite_type = get_sprite_type(&uw_sprite.id);
        if sprite_type == SpriteType::Object {
            // Objects can't be randomized. (Switches, etc.)
            continue;
        }

        // Find the subset of possible choices.
        let mut possible_matches: Vec<&SpriteId> = choices
            .iter()
            .filter(|&it| is_fully_compatible(&uw_sprite.id, it, placement, uw_sprite.has_key()))
            .collect::<Vec<_>>();

        if possible_matches.is_empty() {
            // If there is nothing that is fully compatible, use looser rules
            possible_matches = choices
                .iter()
                .filter(|&it| {
                    is_partially_compatible(&uw_sprite.id, it, placement, uw_sprite.has_key())
                })
                .collect::<Vec<_>>();
            if possible_matches.is_empty() {
                // Mark sprite as removed.
                removed_indexes.push(index);
                continue;
            }
        }

        // Compute the weights.
        let weights = get_weights(&model.uw_balancing, is_adjusted_down, &possible_matches);
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
        let sprite = spritelist.sprites.remove(pos);
        if sprite.has_key() {
            // Kill the program if we don't have a replacement for a sprite with a key.
            // This is very bad and
            panic!(
                "Room {} Sprite {} Index {} has key and couldn't be replaced!",
                room_id, sprite.id, pos
            );
        }
    }
}
