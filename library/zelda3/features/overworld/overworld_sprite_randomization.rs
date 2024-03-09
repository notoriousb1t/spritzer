use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::zelda3::features::sprites::get_weights;
use crate::zelda3::features::sprites::is_compatible;
use crate::zelda3::features::sprites::Placement;
use crate::zelda3::model::can_sprite_fly;
use crate::zelda3::model::can_sprite_swim;
use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::OWRoomState;
use crate::zelda3::model::OWSprite;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::Z3Model;

/// This re-arranges the positions of non-critical enemies.
pub(crate) fn shuffle_overworld_sprites(model: &mut Z3Model) {
    let mut rng = model.create_rng();

    let mut rooms = model.ow_rooms.values_mut().collect::<Vec<_>>();
    rooms.sort_by_key(|room| room.id);

    for room in rooms {
        for version in room.versions_mut() {
            // Grab all sprites that can be shuffled (just enemies that don't hold keys).
            let mut enemies: Vec<&mut OWSprite> = version
                .sprites
                .iter_mut()
                .filter(|sprite| {
                    matches!(
                        get_sprite_type(&sprite.id),
                        SpriteType::Enemy | SpriteType::Hazard
                    ) && !can_sprite_fly(&sprite.id)
                        && !can_sprite_swim(&sprite.id)
                })
                .collect();

            let mut positions: Vec<(u8, u8)> = enemies.iter().map(|it| (it.x, it.y)).collect();
            positions.shuffle(&mut rng);

            for (index, enemy) in enemies.iter_mut().enumerate() {
                let (x, y) = positions[index];
                enemy.x = x;
                enemy.y = y;
            }
        }
    }
}

pub(crate) fn reroll_overworld_sprites(model: &mut Z3Model) {
    let mut rng = model.create_rng();

    let types = vec![SpriteType::Enemy, SpriteType::Creature, SpriteType::Boss];
    let keys = model.ow_rooms.keys().cloned().collect::<Vec<_>>();
    for overworld_id in keys {
        let mut area = model.ow_rooms.get_mut(&overworld_id).unwrap().clone();
        if let Some(version) = &mut area.lw_pre_aga {
            _reroll_overworld_sprites(model, &mut rng, version, &types);
        }
        _reroll_overworld_sprites(model, &mut rng, &mut area.lw, &types);
        if let Some(version) = &mut area.dw {
            _reroll_overworld_sprites(model, &mut rng, version, &types);
        }
        model.ow_rooms.insert(overworld_id, area);
    }
}

fn _reroll_overworld_sprites(
    model: &Z3Model,
    rng: &mut StdRng,
    version: &mut OWRoomState,
    types: &[SpriteType],
) {
    let choices = &model.sprite_pool[&version.spriteset_id];
    if choices.is_empty() {
        return;
    }

    for overworld_sprite in version.sprites.iter_mut() {
        if !types.contains(&get_sprite_type(&overworld_sprite.id)) {
            continue;
        }

        // Find all normal replacements
        let possible_matches = choices
            .iter()
            .filter(|&it| is_compatible(&overworld_sprite.id, it, Placement::Area, false))
            .collect::<Vec<_>>();

        if possible_matches.is_empty() {
            // Medusas have no specific spriteset requirements, so they are
            // used when there is nothing to fill a specific requirement.
            overworld_sprite.id = SpriteId::xC5_MEDUSA;
            continue;
        }

        // Try to find a suitable match, if not just leave the Sprite as is.

        let weights = get_weights(&model.ow_balancing, &possible_matches);
        if let Ok(&sprite_id) =
            possible_matches.choose_weighted(rng, |sprite_id| weights[*sprite_id])
        {
            overworld_sprite.id = *sprite_id;
        }
    }
}
