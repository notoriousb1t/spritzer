use log;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::zelda3::model::filter_compatible;
use crate::zelda3::model::get_overworld_rules;
use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::get_weights;
use crate::zelda3::model::DungeonId;
use crate::zelda3::model::OWRoomId;
use crate::zelda3::model::Rule;
use crate::zelda3::model::Sprite;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::Z3Model;
use crate::zelda3::Balancing;

pub(crate) fn shuffle_overworld_sprites(model: &mut Z3Model) {
    log::info!("Shuffling sprites in overworld");
    model.prepare_sprite_pool();

    let balancing = model.ow_balancing;
    let mut rng = model.create_rng();

    for room_id in OWRoomId::iter() {
        // Recalculate sprites for all versions of the room and get a list of the sprites for each
        // room state and the state id.
        let room = model.ow_rooms.get(&room_id).unwrap();
        let state_sprites = room
            .versions()
            .iter()
            .map(|state| {
                let room_rules = get_overworld_rules(state.overworld_id);
                let spriteset_id = state.spriteset_id;
                let sprites = randomize_sprites(
                    model,
                    &mut rng,
                    spriteset_id,
                    balancing,
                    &room_rules,
                    state.sprites.clone(),
                    format!("Room = {}, {}", room_id, state.overworld_id),
                );
                return (state.overworld_id, sprites);
            })
            .collect::<Vec<_>>();

        let room = model.ow_rooms.get_mut(&room_id).unwrap();
        for (state_id, sprites) in state_sprites {
            let state = room.get_mut(state_id).unwrap();
            state.sprites = sprites;
        }
    }
}

pub(crate) fn shuffle_underworld_sprites(model: &mut Z3Model) {
    log::info!("Shuffling sprites in underworld");
    model.prepare_sprite_pool();

    let balancing = model.uw_balancing;
    let mut rng = model.create_rng();

    for room_id in UWRoomId::iter() {
        // Get the list of choices.
        let header = model
            .uw_headers
            .get(&room_id)
            .expect("UWRoomHeader To exist");

        let spriteset_id = header.spriteset_id;

        // If the room is part of escape or rescue, artificially lower the difficulty because this
        // could be a standard run and there are some rooms that are painful given the
        // number of pests normally there or impossible to kill with just the arrows.
        let is_rescue = model
            .dungeons
            .iter()
            .find(|(dungeon_id, dungeon)| {
                (**dungeon_id == DungeonId::X00_Sewers
                    || **dungeon_id == DungeonId::X02_HyruleCastle)
                    && dungeon.rooms.contains(&room_id)
            })
            .is_some();

        // Get the placement strategy.
        let mut rules = vec![Rule::Underworld];
        if header.tag1.is_kill_room() || header.tag2.is_kill_room() {
            rules.push(Rule::KillRequired);
        };
        if is_rescue {
            rules.push(Rule::ReduceDifficulty);
        }

        // Get the current sprites in the room.
        let sprites = model
            .uw_sprites
            .get_mut(&room_id)
            .expect("Sprites should exist")
            .sprites
            .clone();

        let sprites = randomize_sprites(
            model,
            &mut rng,
            spriteset_id,
            balancing,
            &rules,
            sprites,
            format!("Room = {}", room_id),
        );

        let spritelist = model
            .uw_sprites
            .get_mut(&room_id)
            .expect("Sprites should exist");
        spritelist.sprites = sprites;
    }
}

fn randomize_sprites(
    model: &Z3Model,
    rng: &mut StdRng,
    spriteset_id: SpritesetId,
    balancing: Balancing,
    room_rules: &[Rule],
    sprites: Vec<Sprite>,
    debug_message: String,
) -> Vec<Sprite> {
    if sprites
        .iter()
        .any(|sprite| get_sprite_type(&sprite.id) == SpriteType::Boss)
    {
        // Do not modify boss rooms.
        log::debug!("Skipping boss room. Debug = {}", debug_message);
        return sprites;
    }

    // Get the current sprite choices for the pool.
    let choices = &model.sprite_pool[&spriteset_id];

    return sprites
        .iter()
        .filter_map(|sprite| {
            let mut sprite_rules = room_rules.to_vec();
            if sprite.has_key() {
                sprite_rules.push(Rule::KeyRequired);
            }

            // Find the subset of possible choices.
            let possible_matches = filter_compatible(sprite.id, choices, &sprite_rules);

            if possible_matches.is_empty() {
                if sprite.has_key() {
                    // This is a last resort recovery for keys.
                    // If there is no match for something that can hold a key, it is simply dropped
                    // This was tested in Eastern Palace and the key appears to remain collected after pickup.
                    log::warn!("Key has dropped the Sprite! {} | {}", sprite.id, debug_message);

                    let mut new_sprite = sprite.clone();
                    new_sprite.id = sprite.item.unwrap();
                    new_sprite.aux0 = None;
                    new_sprite.aux1 = None;
                    new_sprite.item = None;
                    return Some(new_sprite);
                }
                return None;
            }

            // Compute the weights.
            let weights = get_weights(&balancing, &sprite_rules, &possible_matches);
            if weights.iter().all(|it| it.1 == &0) {
                return Some(sprite.clone());
            }

            let sprite_result = possible_matches
                .choose_weighted(rng, |it| weights[it])
                .unwrap();

            let mut new_sprite = sprite.clone();
            new_sprite.id = *sprite_result;
            return Some(new_sprite);
        })
        .collect::<Vec<_>>();
}
