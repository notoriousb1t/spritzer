use std::collections::BTreeMap;

use strum::IntoEnumIterator;

use crate::zelda3::model::get_spritesheet_arrangements;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpritesheetId;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::Z3Model;

fn is_spritesheet_satisfied(
    spritesheet_reqs: &[SpritesheetId; 4],
    sprite_requirements: &Vec<[SpritesheetId; 4]>,
) -> bool {
    sprite_requirements.is_empty()
        || sprite_requirements.iter().any(|config| {
            (0..4).all(|i| config[i] == SpritesheetId::None || config[i] == spritesheet_reqs[i])
        })
}

/// Compute sprite choices based on what is possible in a Spriteset.
pub(crate) fn calculate_sprite_pool(model: &Z3Model) -> BTreeMap<SpritesetId, Vec<SpriteId>> {
    let sprite_requirements = SpriteId::iter()
        .map(|sprite_id| (sprite_id, get_spritesheet_arrangements(&sprite_id)))
        .collect::<Vec<_>>();

    BTreeMap::from_iter(model.spritesets.iter().map(|(spriteset_id, spriteset)| {
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
