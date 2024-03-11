mod killable_thieves;
mod moldorm_shuffle;
mod mushroom_shuffle;
mod ow_inversion;
mod ow_simplication;
mod ow_sprite_shuffle;
mod shadow_bees;
mod sprite_base;
mod dungeon;
mod uw_overlord_shuffle;
mod uw_sprite_shuffle;

use crate::zelda3::model::Z3Model;
use crate::zelda3::options::OverworldEnemyShuffle;
use crate::zelda3::options::UnderworldEnemyShuffle;
use crate::zelda3::options::Z3Options;
use log::info;

use self::killable_thieves::apply_killable_thieves;
use self::moldorm_shuffle::apply_moldorm_eye_shuffle;
use self::mushroom_shuffle::apply_mushroom_shuffle;
use self::ow_inversion::apply_ow_inversion;
use self::ow_simplication::apply_ow_simplication;
use self::ow_sprite_shuffle::apply_ow_sprite_reroll;
use self::ow_sprite_shuffle::apply_ow_sprite_shuffle;
use self::shadow_bees::apply_shadow_bees;
use self::sprite_base::apply_base_sprite_changes;
use self::dungeon::apply_boss_shuffle;
use self::uw_overlord_shuffle::apply_uw_overlord_shuffle;
use self::uw_sprite_shuffle::apply_uw_sprites_reroll;
use self::uw_sprite_shuffle::apply_uw_sprites_shuffle;

pub(crate) fn apply_features(model: &mut Z3Model, options: &Z3Options) {
    info!("{}", options);

    // Apply common changes that are needed to make sprites work well in most situations.
    apply_base_sprite_changes(model);

    // This is too small to be an option and has no effect on gameplay, so just enable it
    // unconditionally.
    apply_moldorm_eye_shuffle(model);

    if options.killable_thieves {
        // Make thieves killable (this can happen at any point in this function)
        apply_killable_thieves(model);
    }
    if options.shadow_bees {
        // Why are you punching yourself?
        apply_shadow_bees(model);
    }

    if options.mushroom_shuffle {
        // This must take place before simplification and inversion transformers.
        apply_mushroom_shuffle(model);
    }

    if options.underworld_enemy_shuffle != UnderworldEnemyShuffle::Vanilla
        || options.overworld_enemy_shuffle != OverworldEnemyShuffle::Vanilla
    {
        // This transformer performs some simplification to the overworld to
        // add additional enemy slots and bonk points. Some NPCs may move as a result.
        apply_ow_simplication(model);
    }

    if options.overworld_enemy_shuffle == OverworldEnemyShuffle::Inverted {
        // Invert spritesets between worlds preserving NPCs and objects and the
        // perform a full shuffle as possible. This flips enemies between worlds
        // wherever it is safe. In the future, this may be more curated.
        apply_ow_inversion(model);
    }

    if options.underworld_enemy_shuffle != UnderworldEnemyShuffle::Vanilla
        || options.overworld_enemy_shuffle != OverworldEnemyShuffle::Vanilla
    {
        // Figure out what sprites can go where. This is cached because it is too
        // costly to do every time a sprite is evaluated.
        model.prepare_sprite_pool();
    }

    match options.overworld_enemy_shuffle {
        OverworldEnemyShuffle::Simple => {
            apply_ow_sprite_shuffle(model);
        }
        OverworldEnemyShuffle::Inverted => {
            apply_ow_sprite_reroll(model);
        }
        OverworldEnemyShuffle::Full => {
            apply_ow_sprite_reroll(model);
        }
        OverworldEnemyShuffle::Chaos => {
            apply_ow_sprite_reroll(model);
        }
        OverworldEnemyShuffle::Insanity => {
            apply_ow_sprite_reroll(model);
        }
        _ => {}
    }

    match options.underworld_enemy_shuffle {
        UnderworldEnemyShuffle::Simple => {
            apply_uw_sprites_shuffle(model);
        }
        UnderworldEnemyShuffle::Full => {
            apply_uw_sprites_reroll(model);
        }
        UnderworldEnemyShuffle::Chaos => {
            apply_uw_sprites_reroll(model);
            apply_uw_overlord_shuffle(model);
        }
        UnderworldEnemyShuffle::Insanity => {
            apply_uw_sprites_reroll(model);
            apply_uw_overlord_shuffle(model);
        }
        _ => {}
    }

    if options.boss_shuffle {
        apply_boss_shuffle(model);
    }
}
