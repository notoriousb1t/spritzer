//! This module contains patches to fix Sprites in Zelda3.
//!
//! Example: rollers in Turtle Rock are invulnerable and cannot be killed, but are still considered
//! alive for the purpose of kill rooms where the player must clear all enemies. This was an error
//! in the original game that did not matter to its release since rollers do not appear in kill
//! rooms. However, since this randomizer can place enemies in new situations, sometimes these
//! unimportant bugs or optimizations need to apply to ensure the game doesn't hard/soft lock.

use strum::IntoEnumIterator;

use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::get_sprite_vulnerability;
use crate::zelda3::model::OWRoomId;
use crate::zelda3::model::PaletteIndex;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::SpriteVulnerability;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::SpritesheetId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::Z3Model;

pub(crate) fn apply_base_sprite_changes(model: &mut Z3Model) {
    update_settings(model);
    update_color_index(model);
    update_spritesets(model);
}

/// Applies sprite instance changes across overworld and darkworld. This is largely a find and replace to make
/// shuffles safer or feel better.
pub(crate) fn apply_base_sprite_shuffle_changes(model: &mut Z3Model) {
    // See if there is a required replacement across all versions of all rooms.
    for overworld_id in OWRoomId::iter() {
        if let Some(area) = model.ow_rooms.get_mut(&overworld_id) {
            for sprite in area.sprites_mut() {
                if let Some(new_id) = match sprite.id {
                    // Turns talking trees into bonkable trees. (no spritesheet requirements)
                    SpriteId::x25_TALKING_TREE => Some(SpriteId::xD8_HEART),
                    // Places a bee somewhere nearby a TOPPO location. (no spritesheet requirements)
                    SpriteId::x4D_TOPPO => Some(SpriteId::x79_BEE),
                    // Swaps flopping fish for enemy that is already in area in vanilla.
                    SpriteId::xD2_FLOPPING_FISH => Some(SpriteId::xF_OCTOBALLOON),
                    // Swaps fake master sword for another enemy common to the area in vanilla.
                    SpriteId::xE8_FAKE_MASTER_SWORD => Some(SpriteId::xD_BUZZBLOB),
                    // Swaps all red spear guard 2s for blue guards so they can be removed from shuffle.
                    // Black spear guard is one of two variants of red spear guard in vanilla. Because of its
                    // low spritesheet requirements, it occurs two frequently. The other variant is more flexible
                    // since the head can be swapped with a bunch of spritesheets.
                    SpriteId::x45RedSpearGuard2 => Some(SpriteId::x41BlueSwordGuard),
                    _ => None,
                } {
                    sprite.id = new_id;
                }
            }
        }
    }

    // See if there is a required replacement across all underworld rooms.
    for uw_id in UWRoomId::iter() {
        if let Some(room) = model.uw_sprites.get_mut(&uw_id) {
            for sprite in room.sprites.iter_mut() {
                if let Some(new_id) = match sprite.id {
                    // Same rationale as overworld replacement.
                    SpriteId::x45RedSpearGuard2 => Some(SpriteId::x41BlueSwordGuard),
                    _ => None,
                } {
                    sprite.id = new_id;
                }
            }
        }
    }
}

/// Align expectations between the game and Spritzer.
fn update_settings(model: &mut Z3Model) {
    for sprite in model.sprite_settings.values_mut() {
        let sprite_type = get_sprite_type(&sprite.id);

        // This makes sure the game and randomizer are aligned on what is killable for the
        // purposes of underworld kill rooms.
        sprite.statis = get_sprite_vulnerability(&sprite.id) == SpriteVulnerability::Invulnerable;

        match sprite_type {
            SpriteType::Enemy => {
                // Flag all enemies as being eligible for boss battles.
                sprite.boss_prep_preserved = true;
            }
            SpriteType::Hazard => {
                // Flag all enemies as being eligible for boss battles.
                sprite.boss_prep_preserved = true;
            }
            _ => {
                // Outside of enemies and hazards, preserve existing logic in game.
            }
        }
    }
}

fn update_color_index(model: &mut Z3Model) {
    if let Some(sprite) = model.sprite_settings.get_mut(&SpriteId::x45RedSpearGuard2) {
        // There are two red guards with spears in light world.
        // There are two green guards with spears in the dark world.
        // Assign a different color to differentiate. This creates the iron soldiers which are removed 
        // during randmomization.
        sprite.palette = PaletteIndex::XENoir;
    }
    if let Some(sprite) = model.sprite_settings.get_mut(&SpriteId::x6B_CANNON_GUARD) {
        sprite.palette = PaletteIndex::XDGreen;
    }
    if let Some(sprite) = model.sprite_settings.get_mut(&SpriteId::x13_MINI_HELMASAUR) {
        // Stablize color to something that works better across rooms. (blue)
        sprite.palette = PaletteIndex::XCBlue;
    }
    if let Some(sprite) = model.sprite_settings.get_mut(&SpriteId::x9B_WIZZROBE) {
        // Stablize color to something that works better across rooms. (red)
        sprite.palette = PaletteIndex::XBRed;
    }
    if let Some(sprite) = model.sprite_settings.get_mut(&SpriteId::x83_GREEN_EYEGORE) {
        // Stablize color to something that works better across rooms. (blue)
        sprite.palette = PaletteIndex::XCBlue;
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
    zora_king_set.sheets[1] = SpritesheetId::None;
}
