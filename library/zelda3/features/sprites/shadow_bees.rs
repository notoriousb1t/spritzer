use crate::zelda3::model::SpriteId;
use crate::zelda3::model::Z3Model;

pub(crate) fn patch_shadow_bees(model: &mut Z3Model) {
    if let Some(bee) = model.sprite_settings.get_mut(&SpriteId::x79_BEE) {
        bee.display_allocation = 0;
        bee.draw_shadow = true;
        bee.statis = true;
        bee.boss_damage_sfx = true;
        bee.hp = 0x16;
    }
}
