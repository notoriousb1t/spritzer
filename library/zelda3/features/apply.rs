use log::info;

use crate::zelda3::features::overworld::invert_world;
use crate::zelda3::features::overworld::reroll_lost_woods_mushroom;
use crate::zelda3::features::overworld::reroll_overworld_sprites;
use crate::zelda3::features::overworld::shuffle_overworld_sprites;
use crate::zelda3::features::overworld::simplify_overworld;
use crate::zelda3::features::sprites::apply_base_sprite_changes;
use crate::zelda3::features::sprites::patch_killable_thieves;
use crate::zelda3::features::sprites::patch_shadow_bees;
use crate::zelda3::features::underworld::reroll_underworld_overlords;
use crate::zelda3::features::underworld::reroll_underworld_sprites;
use crate::zelda3::features::underworld::shuffle_bosses;
use crate::zelda3::features::underworld::shuffle_underworld_sprites;
use crate::zelda3::features::underworld::update_moldorm_eyes;
use crate::zelda3::model::Z3Model;
use crate::zelda3::options::OverworldEnemyShuffle;
use crate::zelda3::options::UnderworldEnemyShuffle;
use crate::zelda3::options::Z3Options;

pub(crate) fn apply_options(model: &mut Z3Model, options: &Z3Options) {
    info!("{}", options);

    apply_base_sprite_changes(model);

    if options.mushroom_shuffle {
        // This must take place before simplification and inversion transformers.
        reroll_lost_woods_mushroom(model);
    }
    if options.shadow_bees {
        patch_shadow_bees(model);
    }

    if options.underworld_enemy_shuffle != UnderworldEnemyShuffle::Vanilla
        || options.overworld_enemy_shuffle != OverworldEnemyShuffle::Vanilla
    {
        // This transformer performs some simplification to the overworld to
        // add additional enemy slots and bonk points. Some NPCs may move as a result.
        simplify_overworld(model);
        patch_killable_thieves(model);
    }

    if options.overworld_enemy_shuffle == OverworldEnemyShuffle::Inverted {
        // Invert spritesets between worlds preserving NPCs and objects and the
        // perform a full shuffle as possible. This flips enemies between worlds
        // wherever it is safe. In the future, this may be more curated.
        invert_world(model);
    }

    if options.underworld_enemy_shuffle != UnderworldEnemyShuffle::Vanilla
        || options.overworld_enemy_shuffle != OverworldEnemyShuffle::Vanilla
    {
        model.prepare_sprite_pool();
    }

    match options.overworld_enemy_shuffle {
        OverworldEnemyShuffle::Vanilla => {}
        OverworldEnemyShuffle::Simple => {
            shuffle_overworld_sprites(model);
        }
        OverworldEnemyShuffle::Inverted => {
            reroll_overworld_sprites(model);
        }
        OverworldEnemyShuffle::Full => {
            reroll_overworld_sprites(model);
        }
        OverworldEnemyShuffle::Chaos => {
            reroll_overworld_sprites(model);
        }
        OverworldEnemyShuffle::Insanity => {
            reroll_overworld_sprites(model);
        }
    }

    match options.underworld_enemy_shuffle {
        UnderworldEnemyShuffle::Vanilla => {}
        UnderworldEnemyShuffle::Simple => {
            shuffle_underworld_sprites(model);
        }
        UnderworldEnemyShuffle::Full => {
            reroll_underworld_sprites(model);
        }
        UnderworldEnemyShuffle::Chaos => {
            reroll_underworld_sprites(model);
            reroll_underworld_overlords(model);
        }
        UnderworldEnemyShuffle::Insanity => {
            reroll_underworld_sprites(model);
            reroll_underworld_overlords(model);
        }
    }

    match options.boss_shuffle {
        true => {
            update_moldorm_eyes(model);
            shuffle_bosses(model);
        }
        false => {}
    }
}
