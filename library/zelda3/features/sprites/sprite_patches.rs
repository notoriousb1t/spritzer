//! This module contains patches to fix Sprites in Zelda3.
//!
//! Example: rollers in Turtle Rock are invulnerable and cannot be killed, but are still considered
//! alive for the purpose of kill rooms where the player must clear all enemies. This was an error
//! in the original game that did not matter to its release since rollers do not appear in kill
//! rooms. However, since this randomizer can place enemies in new situations, sometimes these
//! unimportant bugs or optimizations need to apply to ensure the game doesn't hard/soft lock.

use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::get_sprite_vulnerability;
use crate::zelda3::model::PaletteIndex;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteSheetId;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::SpriteVulnerability;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::Z3Model;

pub(crate) fn apply_base_sprite_changes(model: &mut Z3Model) {
    update_settings(model);
    update_color_index(model);
    update_spritesets(model);
}

/// Align expectations between the game and Spritzer.
fn update_settings(model: &mut Z3Model) {
    for sprite in model.sprite_settings.values_mut() {
        let sprite_type = get_sprite_type(&sprite.id);

        if sprite_type == SpriteType::Hazard
            || sprite.id == SpriteId::xD8_HEART
            || get_sprite_vulnerability(&sprite.id) == SpriteVulnerability::Invulnerable
        {
            // This makes sure the game and randomizer are aligned on what is killable for the
            // purposes of underworld kill rooms.
            sprite.statis = true;
        }

        match sprite_type {
            SpriteType::Enemy => {
                // Flag all enemies as being eligible for boss battles.
                sprite.boss_prep_preserved = true;
            }
            SpriteType::Hazard => {
                // Flag all enemies as being eligible for boss battles.
                sprite.boss_prep_preserved = true;
            }
            _ => {}
        }
    }
}

fn update_color_index(model: &mut Z3Model) {
    if let Some(sprite) = model
        .sprite_settings
        .get_mut(&SpriteId::x45_BLACK_SPEAR_GUARD)
    {
        // There are two red guards with spears in light world.
        // There are two green guards with spears in the dark world.
        // Assign a different color to differentiate. This creates the iron soldiers.
        sprite.palette = PaletteIndex::XENoir;
    }
    if let Some(sprite) = model.sprite_settings.get_mut(&SpriteId::x6B_CANNON_GUARD) {
        // This isn't in the vanilla game so pick a more distingishable color.
        sprite.palette = PaletteIndex::XENoir;
    }
    if let Some(sprite) = model.sprite_settings.get_mut(&SpriteId::x13_MINI_HELMASAUR) {
        // Stablize color to something that works better across rooms.
        sprite.palette = PaletteIndex::XC;
    }
    if let Some(sprite) = model.sprite_settings.get_mut(&SpriteId::x9B_WIZZROBE) {
        // Stablize color to something that works better across rooms.
        sprite.palette = PaletteIndex::XB;
    }
    if let Some(sprite) = model.sprite_settings.get_mut(&SpriteId::x83_GREEN_EYEGORE) {
        // Stablize color to something that works better across rooms.
        sprite.palette = PaletteIndex::XC;
    }
}

fn update_spritesets(model: &mut Z3Model) {
    // Get the zora king set and change spritesheet slot 1 to use something else.
    // Without this change, the game ends up with a lot of Goriya which don't work well
    // in Zora's domain.
    let zora_king_set = model
        .spritesets
        .get_mut(&SpritesetId::x0E_ZORAS_DOMAIN)
        .expect("Expected to find Zora king set");
    zora_king_set.sheets[1] = SpriteSheetId::None;
}
