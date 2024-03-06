use assembly::zelda3::Symbol;

use crate::snes::Patch;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::Z3Model;

pub(crate) fn patch_killable_thieves(model: &mut Z3Model) {
    model.patches.push(Patch::of(
        Symbol::Killable as usize + 4,
        SpriteId::xC4_THIEF as u8,
    ));

    if let Some(thief) = model.sprite_settings.get_mut(&SpriteId::xC4_THIEF) {
        thief.hp = 6;
    }
}
