mod dungeon;
mod killable_thieves;
mod moldorm_shuffle;
mod mushroom_shuffle;
mod ow_inversion;
mod shadow_bees;
mod sprite_base;
mod sprite_shuffle;
mod spriteset_shuffle;
mod uw_overlord_shuffle;

use log;

use self::dungeon::apply_boss_shuffle;
use self::killable_thieves::apply_killable_thieves;
use self::moldorm_shuffle::apply_moldorm_eye_shuffle;
use self::mushroom_shuffle::apply_mushroom_shuffle;
use self::ow_inversion::apply_ow_inversion;
use self::shadow_bees::apply_shadow_bees;
use self::sprite_base::apply_base_sprite_changes;
use self::sprite_shuffle::shuffle_underworld_sprites;
use self::spriteset_shuffle::shuffle_overworld_spritesets;
use self::spriteset_shuffle::shuffle_underworld_spritesets;
use self::uw_overlord_shuffle::apply_uw_overlord_shuffle;
use crate::zelda3::features::sprite_base::apply_base_sprite_shuffle_changes;
use crate::zelda3::features::sprite_shuffle::shuffle_overworld_sprites;
use crate::zelda3::model::Z3Model;
use crate::zelda3::options::OverworldEnemyShuffle;
use crate::zelda3::options::UnderworldEnemyShuffle;
use crate::zelda3::options::Z3Options;

pub(crate) fn apply_features(model: &mut Z3Model, options: &Z3Options) {
    log::info!("{}", options);

    let sprites_will_change = options.underworld_enemy_shuffle != UnderworldEnemyShuffle::Vanilla
        || options.overworld_enemy_shuffle != OverworldEnemyShuffle::Vanilla
        || options.boss_shuffle;

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

    if sprites_will_change {
        // This transformer performs some simplification to the overworld to
        // add additional enemy slots and bonk points. Some NPCs may move as a result.
        apply_base_sprite_shuffle_changes(model);
    }

    if options.overworld_enemy_shuffle == OverworldEnemyShuffle::Inverted {
        // Invert spritesets between worlds preserving NPCs and objects and the
        // perform a full shuffle as possible. This flips enemies between worlds
        // wherever it is safe. In the future, this may be more curated.
        apply_ow_inversion(model);
    }

    if options.boss_shuffle {
        apply_boss_shuffle(model);
    }

    // Process spriteset shuffling if applicable for Underworld.
    // Do this first because overworld can use underworld, but not the inverse.
    // Doing this first provides more options for overworld.
    match options.underworld_enemy_shuffle {
        UnderworldEnemyShuffle::Chaos => {
            shuffle_overworld_spritesets(model);
        }
        UnderworldEnemyShuffle::Insanity => {
            shuffle_overworld_spritesets(model);
        }
        _ => {}
    }

    // Process spriteset shuffling if applicable for Overworld.
    match options.overworld_enemy_shuffle {
        OverworldEnemyShuffle::Chaos => {
            shuffle_underworld_spritesets(model);
        }
        OverworldEnemyShuffle::Insanity => {
            shuffle_underworld_spritesets(model);
        }
        _ => {}
    }

    match options.underworld_enemy_shuffle {
        UnderworldEnemyShuffle::Full => {
            shuffle_underworld_sprites(model);
        }
        UnderworldEnemyShuffle::Chaos => {
            shuffle_underworld_sprites(model);
            shuffle_overworld_spritesets(model);
        }
        UnderworldEnemyShuffle::Insanity => {
            shuffle_underworld_sprites(model);
            apply_uw_overlord_shuffle(model);
        }
        _ => {}
    }

    match options.overworld_enemy_shuffle {
        OverworldEnemyShuffle::Inverted => {
            shuffle_overworld_sprites(model);
        }
        OverworldEnemyShuffle::Full => {
            shuffle_overworld_sprites(model);
        }
        OverworldEnemyShuffle::Chaos => {
            shuffle_overworld_sprites(model);
        }
        OverworldEnemyShuffle::Insanity => {
            shuffle_overworld_sprites(model);
        }
        _ => {}
    }
}
