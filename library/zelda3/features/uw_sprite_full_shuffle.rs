//! TODO: fix this mess. There is a ton of cloning here and general nastiness
//! because of mutable borrows. Much of this is a result of porting from questionable
//! choices in the initial Python implementation.

use std::collections::BTreeMap;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::zelda3::model::can_shuffle_in_uw;
use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::get_weights;
use crate::zelda3::model::is_fully_compatible;
use crate::zelda3::model::Placement;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::UWSprite;
use crate::zelda3::model::UWSpriteList;
use crate::zelda3::model::Z3Model;

pub(crate) fn apply_uw_sprites_full_shuffle(model: &mut Z3Model) {
    let mut rng = model.create_rng();

    let ids = model.uw_sprites.keys().cloned().collect::<Vec<_>>();
    for id in ids {
        let room = model.uw_sprites.get(&id).unwrap();
        let room = reroll_underworld_room(model, &mut rng, room);
        model.uw_sprites.insert(id, room);
    }
}

fn find_distance(start: (u8, u8), end: (u8, u8)) -> f32 {
    (((start.0 as i16 - end.0 as i16).pow(2) + (start.1 as i16 - end.1 as i16).pow(2)) as f32)
        .powf(0.5)
}

fn detect_room_configuration(dungeon_sprites: &[&UWSprite]) -> (i32, i32, bool) {
    let mut start_x: i32 = 0;
    let mut start_y: i32 = 0;
    let mut end_x: i32 = 0;
    let mut end_y: i32 = 0;
    let mut max_distance = 0.0;
    for i in 0..dungeon_sprites.len() {
        for j in (i + 1)..dungeon_sprites.len() {
            let distance = find_distance(
                (dungeon_sprites[i].x_pos, dungeon_sprites[i].y_pos),
                (dungeon_sprites[j].x_pos, dungeon_sprites[j].y_pos),
            );
            if distance <= max_distance && max_distance != 0.0 {
                continue;
            }
            max_distance = distance;
            let start_sprite = &dungeon_sprites[if distance > 0.0 { i } else { j }];
            start_x = start_sprite.x_pos as i32;
            start_y = start_sprite.y_pos as i32;
            let end_sprite = &dungeon_sprites[if distance <= 0.0 { i } else { j }];
            end_x = end_sprite.x_pos as i32;
            end_y = end_sprite.y_pos as i32;
        }
    }

    let is_horizontal: bool = (start_x - end_x) as u16 > (start_y - end_y) as u16;
    (end_x - start_x, end_y - start_y, is_horizontal)
}

fn sort_by_distance(original_sprites: &[UWSprite]) -> Vec<UWSprite> {
    let mut sprites = original_sprites.to_owned();
    if sprites.len() < 2 {
        return sprites;
    }

    // Sort the sprites so they are evaluated based on distance from the
    // center. This should place sprites nearest to the center in the
    // beginning of the list and further away ones at the end of the list
    // absolute value is uses so we can match up enemies that are symmetrical
    sprites.sort_by_key(|it| it.distance_from_midpoint);

    sprites.to_vec()
}

fn compute_distance(config: (i32, i32, bool), original_sprites: &[&UWSprite]) -> Vec<UWSprite> {
    let mut sprites: Vec<UWSprite> = vec![];
    for &dungeon_sprite in original_sprites.iter() {
        let mut sprite = *dungeon_sprite;
        if config.2 {
            // Use the distance from midpoint on the y axis for horizontal configs.
            sprite.distance_from_midpoint = dungeon_sprite.y_pos as i32 - config.1;
        } else {
            // Use the distance from midpoint on the x axis for vertical configs.
            sprite.distance_from_midpoint = dungeon_sprite.x_pos as i32 - config.0;
        }
        sprites.push(sprite);
    }
    sprites
}

fn generate_sprite_selections(
    model: &Z3Model,
    rng: &mut StdRng,
    dungeon_room_sprites: Vec<UWSprite>,
    choices: &[SpriteId],
    placement: Placement,
) -> BTreeMap<i32, SpriteId> {
    let mut distance_map: BTreeMap<i32, SpriteId> = BTreeMap::new();
    let dungeon_sprite_groups = [
        dungeon_room_sprites
            .iter()
            .filter(|&it| it.has_key())
            .collect::<Vec<_>>(),
        dungeon_room_sprites
            .iter()
            .filter(|&it| !it.has_key())
            .collect::<Vec<_>>(),
    ];
    for dungeon_room_sprite_group in dungeon_sprite_groups {
        // Evaluate enemies with keys first so we can make sure that we always have keys accounted
        // for This ensures that if a key is required as part of a distance pair, that the
        // sprite selection makes sure a key is in that group.
        for dungeon_sprite in dungeon_room_sprite_group {
            if distance_map.contains_key(&dungeon_sprite.distance_from_midpoint) {
                continue;
            }

            // Find all normal replacements. Make sure to include the sprite already present.
            let possible_matches: Vec<&SpriteId> = choices
                .iter()
                .filter(|&it| {
                    is_fully_compatible(&dungeon_sprite.id, it, placement, dungeon_sprite.has_key())
                })
                .collect::<Vec<_>>();

            if possible_matches.is_empty() {
                distance_map.insert(dungeon_sprite.distance_from_midpoint, dungeon_sprite.id);
                continue;
            }

            let weights = get_weights(&model.uw_balancing, &possible_matches);
            if weights.iter().all(|it| it.1 == &0) {
                // If all weights are zero, continue. This signifies no valid replacements.
                // Just replace with self in that case.
                distance_map.insert(dungeon_sprite.distance_from_midpoint, dungeon_sprite.id);
                continue;
            }

            let sprite_result = possible_matches
                .choose_weighted(rng, |&it| weights[it])
                .unwrap();

            distance_map.insert(dungeon_sprite.distance_from_midpoint, **sprite_result);
        }
    }
    distance_map
}

fn reroll_underworld_sprites(
    model: &Z3Model,
    rng: &mut StdRng,
    original_sprites: &[&UWSprite],
    choices: &[SpriteId],
    placement: Placement,
) -> Vec<UWSprite> {
    // Find the center point between all sprites in this list.
    let config = detect_room_configuration(original_sprites);
    let sprites = compute_distance(config, original_sprites);
    // Presort Dungeon Room Sprites by distance from the center.
    let mut sprites2 = sort_by_distance(&sprites);
    let distance_map = generate_sprite_selections(model, rng, sprites, choices, placement);

    let mut return_sprites: Vec<UWSprite> = vec![];
    sprites2.iter_mut().for_each(|original_sprites| {
        let mut sprite = original_sprites.to_owned();
        let next_sprite_id = distance_map[&sprite.distance_from_midpoint];
        if next_sprite_id != sprite.id {
            // Clear aux data because it may be unpredictable.
            sprite.aux0 = 0;
            sprite.aux1 = 0;
        }
        sprite.id = next_sprite_id;
        return_sprites.push(sprite);
    });
    return_sprites
}

fn reroll_underworld_room(
    model: &Z3Model,
    rng: &mut StdRng,
    original_room: &UWSpriteList,
) -> UWSpriteList {
    let mut underworld_room = original_room.clone();

    // Randomize using Entities that occur anywhere in that Dungeon Room.
    if underworld_room
        .sprites
        .iter()
        .any(|it| get_sprite_type(&it.id) == SpriteType::Boss)
    {
        // Skip all boss rooms, we shouldn't try to reroll those through this option.
        return underworld_room;
    }

    let header = model.uw_headers.get(&original_room.room_id).unwrap();
    let choices = &model.sprite_pool[&header.spriteset_id];
    if choices.is_empty() {
        // Skip if there is nothing to switch.
        return underworld_room;
    }

    let placement = match header.tag1.is_kill_room() || header.tag2.is_kill_room() {
        true => Placement::KillRoom,
        false => Placement::Room,
    };

    let mut sprite_ids = SpriteType::iter().collect::<Vec<_>>();
    sprite_ids.sort();

    let mut dungeon_sprites_by_type: BTreeMap<SpriteType, Vec<&UWSprite>> =
        BTreeMap::from_iter(sprite_ids.iter().map(|it| (*it, Vec::new())));
    let mut new_sprites: Vec<UWSprite> = vec![];

    let mut sorted_sprites = underworld_room.sprites.iter().collect::<Vec<_>>();
    sorted_sprites.sort_by_key(|it| (it.y_pos, it.x_pos));

    for sprite in sorted_sprites.iter() {
        if can_shuffle_in_uw(&sprite.id) {
            if let Some(list) = dungeon_sprites_by_type.get_mut(&get_sprite_type(&sprite.id)) {
                list.push(sprite);
            }
        } else {
            new_sprites.push(**sprite);
        }
    }

    for sprite_id in sprite_ids {
        let sprites = dungeon_sprites_by_type.get(&sprite_id).unwrap();
        for sprite in reroll_underworld_sprites(model, rng, sprites, choices, placement) {
            new_sprites.push(sprite);
        }
    }

    underworld_room.sprites = new_sprites;
    underworld_room
}
