use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::zelda3::model::get_sprite_requirements;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteSheetId;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::Z3Model;

fn is_spritesheet_satisfied(
    spritesheet_reqs: &[SpriteSheetId; 4],
    sprite_requirements: &Vec<[SpriteSheetId; 4]>,
) -> bool {
    sprite_requirements.is_empty()
        || sprite_requirements.iter().any(|config| {
            (0..4).all(|i| config[i] == SpriteSheetId::None || config[i] == spritesheet_reqs[i])
        })
}

/// Compute sprite choices based on what is possible in a Spriteset.
pub(crate) fn calculate_sprite_pool(model: &Z3Model) -> HashMap<SpritesetId, Vec<SpriteId>> {
    let sprite_requirements = SpriteId::iter()
        .map(|sprite_id| (sprite_id, get_sprite_requirements(&sprite_id)))
        .collect::<Vec<_>>();

    HashMap::from_iter(model.spritesets.iter().map(|(spriteset_id, spriteset)| {
        (
            *spriteset_id,
            sprite_requirements
                .iter()
                .filter_map(|(sprite_id, sprite_req)| {
                    match sprite_id != &SpriteId::x3_NONE
                        && is_spritesheet_satisfied(&spriteset.sheets, sprite_req)
                    {
                        true => Some(*sprite_id),
                        false => None,
                    }
                })
                .collect::<Vec<_>>(),
        )
    }))
}
