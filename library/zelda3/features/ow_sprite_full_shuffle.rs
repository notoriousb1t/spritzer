use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::get_weights;
use crate::zelda3::model::is_fully_compatible;
use crate::zelda3::model::is_partially_compatible;
use crate::zelda3::model::OWRoomState;
use crate::zelda3::model::Placement;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::Z3Model;

pub(crate) fn apply_ow_sprite_full_shuffle(model: &mut Z3Model) {
    let mut rng = model.create_rng();

    let types = vec![
        SpriteType::Absorbable,
        SpriteType::Boss,
        SpriteType::Creature,
        SpriteType::Enemy,
        SpriteType::Hazard,
    ];
    let keys = model.ow_rooms.keys().cloned().collect::<Vec<_>>();
    for overworld_id in keys {
        let mut area = model.ow_rooms.get_mut(&overworld_id).unwrap().clone();
        reroll_sprites(model, &mut rng, &mut area.lw, &types);
        if let Some(version) = &mut area.lw_post_aga {
            reroll_sprites(model, &mut rng, version, &types);
        }
        if let Some(version) = &mut area.dw {
            reroll_sprites(model, &mut rng, version, &types);
        }
        model.ow_rooms.insert(overworld_id, area);
    }
}

fn reroll_sprites(
    model: &Z3Model,
    rng: &mut StdRng,
    version: &mut OWRoomState,
    types: &[SpriteType],
) {
    let choices = &model.sprite_pool[&version.spriteset_id];
    if choices.is_empty() {
        return;
    }

    // Loop through all sprites, marking sprites as needing removal.
    let mut removed_indexes: Vec<usize> = vec![];
    for (index, ow_sprite) in version.sprites.iter_mut().enumerate() {
        if !types.contains(&get_sprite_type(&ow_sprite.id)) {
            continue;
        }

        // Find all normal replacements
        let mut possible_matches = choices
            .iter()
            .filter(|&it| is_fully_compatible(&ow_sprite.id, it, Placement::Area, false))
            .collect::<Vec<_>>();

        if possible_matches.is_empty() {
            // If there is nothing that is fully compatible, use looser rules
            possible_matches = choices
                .iter()
                .filter(|&it| {
                    is_partially_compatible(&ow_sprite.id, it, Placement::Area, false)
                })
                .collect::<Vec<_>>();
            if possible_matches.is_empty() {
                // Mark sprite as removed.
                removed_indexes.push(index);
                continue;
            }
        }

        // Try to find a suitable match, if not just leave the Sprite as is.

        let weights = get_weights(&model.ow_balancing, false, &possible_matches);
        if let Ok(&sprite_id) =
            possible_matches.choose_weighted(rng, |sprite_id| weights[*sprite_id])
        {
            ow_sprite.id = *sprite_id;
        }
    }

    // Remove any sprites that couldn't be replaced.
    while let Some(pos) = removed_indexes.pop() {
        version.sprites.remove(pos);
    }
}
