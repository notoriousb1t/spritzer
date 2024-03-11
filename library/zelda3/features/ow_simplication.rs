use strum::IntoEnumIterator;

use crate::zelda3::model::OWRoomId;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::Z3Model;

pub(crate) fn apply_ow_simplication(model: &mut Z3Model) {
    replace_problematic_sprites(model);
}

fn replace_problematic_sprites(model: &mut Z3Model) {
    for overworld_id in OWRoomId::iter() {
        if let Some(area) = model.ow_rooms.get_mut(&overworld_id) {
            for sprite in area.sprites_mut() {
                if sprite.id == SpriteId::x25_TALKING_TREE {
                    // Turns talking trees into bonkable trees. (no sprite requirements)
                    sprite.id = SpriteId::xE3_FAIRY
                }
                if sprite.id == SpriteId::x4D_TOPPO {
                    // Places a bee somewhere nearby a TOPPO location. (no sprite requirements)
                    sprite.id = SpriteId::x79_BEE
                }
                if sprite.id == SpriteId::xD2_FLOPPING_FISH {
                    // Swaps flopping fish for enemy that is already in area.
                    sprite.id = SpriteId::xF_OCTOBALLOON
                }
                if sprite.id == SpriteId::xE8_FAKE_MASTER_SWORD {
                    // Swaps fake master sword for another enemy common to the area.
                    sprite.id = SpriteId::xD_BUZZBLOB;
                }
            }
        }
    }
}
