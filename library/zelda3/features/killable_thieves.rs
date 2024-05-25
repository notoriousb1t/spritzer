use assembly::zelda3::Symbol;
use common::Patch;

use crate::zelda3::model::SpriteId;
use crate::zelda3::model::Z3Model;

pub(crate) fn apply_killable_thieves(model: &mut Z3Model) {
    model.patches.push(Patch::of(
        Symbol::EnableKillableThief.into(),
        0x1,
    ));

    if let Some(thief) = model.sprite_settings.get_mut(&SpriteId::xC4_THIEF) {
        thief.hp = 4;
    }
}
