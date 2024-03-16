use rand::seq::SliceRandom;

use crate::zelda3::model::can_sprite_fly;
use crate::zelda3::model::can_sprite_swim;
use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::UWSprite;
use crate::zelda3::model::Z3Model;

/// This re-arranges the positions of non-critical enemies.
pub(crate) fn apply_uw_sprites_simple_shuffle(model: &mut Z3Model) {
    let mut rng = model.create_rng();

    for room in model.uw_sprites.values_mut() {
        // Grab all sprites that can be shuffled (just enemies that don't hold keys).
        let mut enemies: Vec<&mut UWSprite> = room
            .sprites
            .iter_mut()
            .filter(|sprite| {
                !sprite.has_key()
                    && matches!(
                        get_sprite_type(&sprite.id),
                        SpriteType::Enemy | SpriteType::Hazard
                    )
                    && get_sprite_type(&sprite.id) == SpriteType::Enemy
                    && !can_sprite_fly(&sprite.id)
                    && !can_sprite_swim(&sprite.id)
            })
            .collect();

        let mut positions = enemies
            .iter()
            .map(|it| (it.x_pos, it.y_pos, it.lower_layer))
            .collect::<Vec<_>>();
        positions.shuffle(&mut rng);

        for (index, enemy) in enemies.iter_mut().enumerate() {
            let (x, y, lower_layer) = positions[index];
            enemy.x_pos = x;
            enemy.y_pos = y;
            enemy.lower_layer = lower_layer;
        }
    }
}
