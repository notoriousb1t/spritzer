use rand::seq::SliceRandom;

use crate::zelda3::model::can_sprite_fly;
use crate::zelda3::model::can_sprite_swim;
use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::OWSprite;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::Z3Model;

/// This re-arranges the positions of non-critical enemies.
pub(crate) fn apply_ow_sprite_shuffle(model: &mut Z3Model) {
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
