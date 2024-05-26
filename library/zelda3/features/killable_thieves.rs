use crate::zelda3::model::SpriteId;
use crate::zelda3::model::Z3Model;

pub(crate) fn apply_killable_thieves(model: &mut Z3Model) {
    model.game_settings.is_killable_thief = true;
    // TODO: move this into ASM since it can be applied unconditionally.
    if let Some(thief) = model.sprite_settings.get_mut(&SpriteId::xC4_THIEF) {
        thief.hp = 4;
    }
}
