//! This module performs an inversion of the overworld and swaps some spritesheets for underworld
//! rooms associated with overworld enemies. For example, soldiers are swapped with dark world
//! soldiers.

use std::collections::BTreeMap;

use strum::IntoEnumIterator;

use crate::zelda3::model::OWRoom;
use crate::zelda3::model::OWRoomId;
use crate::zelda3::model::OWSprite;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteSheetId;
use crate::zelda3::model::Spriteset;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::Z3Model;

pub(crate) fn invert_world(model: &mut Z3Model) {
    move_salesman(model);
    invert_kakariko(model);
    flip_dungeon_soldiers(model);
    invert_rooms(model);
}

/// Remove the bottle salesman and move to lumberjack house.
/// This makes inverted mode and advanced shuffles a lot simpler.
fn move_salesman(model: &mut Z3Model) {
    let lumberjacks_house = model
        .ow_rooms
        .get_mut(&OWRoomId::x2_LUMBER_JACK_HOUSE)
        .expect("Lumberjack House should exist");

    let bottle_salesman = OWSprite {
        x: 21,
        y: 20,
        id: SpriteId::x75_BOTTLE_SALESMAN,
    };

    if let Some(lumberjacks_house_pre_aga) = lumberjacks_house.lw_pre_aga.as_mut() {
        lumberjacks_house_pre_aga
            .sprites
            .push(bottle_salesman.clone());
    }
    lumberjacks_house.lw.sprites.push(bottle_salesman);

    if let Some(kakariko) = model.ow_rooms.get_mut(&OWRoomId::x18_KAKARIKO_VILLAGE) {
        // Add a thief to the same spot in DW where the salesman is in lightworld.
        if let Some(sprite_pos) = kakariko
            .lw
            .sprites
            .iter_mut()
            .position(|it| it.id == SpriteId::x75_BOTTLE_SALESMAN)
        {
            if let Some(dw) = &mut kakariko.dw {
                let mut thief: OWSprite = kakariko.lw.sprites.remove(sprite_pos);
                thief.id = SpriteId::xC4_THIEF;
                dw.sprites.push(thief);
            }
        }
    }
}

fn invert_kakariko(model: &mut Z3Model) {
    let kak_18 = model
        .ow_rooms
        .get_mut(&OWRoomId::x18_KAKARIKO_VILLAGE)
        .expect("Kakariko Village should exist");

    // Swap all sprites.
    if let Some(dw) = &mut kak_18.dw {
        let mut dw_sprites = dw.sprites.clone();
        let mut lw_sprites = kak_18.lw.sprites.clone();

        if let Some(gate_position) = dw_sprites
            .iter()
            .position(|it| it.id == SpriteId::x14_THIEVES_TOWN_GRATE)
        {
            // Move the Gargoyle Gate to light world before the swap, since it is required
            // in order to gain access to Thieves Town.
            let gate = dw_sprites.remove(gate_position);
            lw_sprites.push(gate);
        }

        kak_18.lw.sprites = dw_sprites;
        dw.sprites = lw_sprites;
    }

    for area_id in [
        OWRoomId::x18_KAKARIKO_VILLAGE,
        OWRoomId::x19_KAKARIKO_VILLAGE,
        OWRoomId::x20_KAKARIKO_VILLAGE,
        OWRoomId::x21_KAKARIKO_VILLAGE,
    ] {
        // It seems that without this, loading into kakariko from a dungeon room in any tile other
        // than the northeast corner can cause unpredictable spritesheets to be loaded. So this
        // guarantees all spritesets are the same between each room header for kakariko's overworld.
        if let Some(area) = model.ow_rooms.get_mut(&area_id) {
            swap_spritesets(area);
        }
    }
}

/// Switch any soldiers in dungeons to use the dark world soldier (as possible). This largely
/// affects the underworld normally associated with the light world.
fn flip_dungeon_soldiers(model: &mut Z3Model) {
    for (spriteset_id, spriteset) in model.spritesets.iter_mut() {
        if spriteset_id >= &SpritesetId::x40_FREESPACE
            && spriteset.sheets[1] == SpriteSheetId::x49_SOLDIERS
        {
            spriteset.sheets[1] = SpriteSheetId::xD_SOLDIERS_DW;
        }
    }
}

fn invert_rooms(model: &mut Z3Model) {
    for overworld_id in OWRoomId::iter() {
        if let Some(area) = model.ow_rooms.get_mut(&overworld_id) {
            if overworld_id == OWRoomId::x18_KAKARIKO_VILLAGE
                || overworld_id == OWRoomId::x19_KAKARIKO_VILLAGE
                || overworld_id == OWRoomId::x20_KAKARIKO_VILLAGE
                || overworld_id == OWRoomId::x21_KAKARIKO_VILLAGE
            {
                continue;
            }
            if let Some(configuration) = get_overworld_configuration(overworld_id) {
                invert_special_overworld_versions(
                    configuration,
                    &mut model.spritesets,
                    &mut model.ow_spritesets_unused,
                    area,
                )
            } else {
                swap_spritesets(area);
            }
        }
    }

    for overworld_id in [OWRoomId::x10_PATH_BETWEEN_KAKARIKO_VILLAGE_AND_LOST_WOODS] {
        if let Some(area) = model.ow_rooms.get_mut(&overworld_id) {
            if let Some(version) = &mut area.dw {
                for sprite in &mut version.sprites {
                    // This swap doesn't work as well because kakariko has no flying creatures.
                    // Replace the poe here with a Cucco.
                    if sprite.id == SpriteId::x19_POE {
                        sprite.id = SpriteId::xB_CUCCO
                    }
                }
            }
        }
    }

    // Restores Poes in Dark Kakariko.
    if let Some(spriteset) = model.spritesets.get_mut(&SpritesetId::x15_THIEVES_VILLAGE) {
        spriteset.sheets[3] = SpriteSheetId::x15_POE_THIEF_DW;
    }
}

fn invert_special_overworld_versions(
    configuration: (Vec<usize>, Vec<usize>),
    spritesets: &mut BTreeMap<SpritesetId, Spriteset>,
    unused_spritesets: &mut Vec<SpritesetId>,
    area: &mut OWRoom,
) {
    if area.dw.is_none() {
        // Skip inversion for areas that don't have both a light and dark world.
        // It is assumed that light world v2 exists if any light world exists
        return;
    }

    let dw = area
        .dw
        .as_mut()
        .expect(&format!("{} should have Dark World", area.id));

    // Specific patches to preserve NPCs and objects, but non-restrictive overall.
    let (preserved_lw_sheets, preserved_dw_sheets) = &configuration;
    let spriteset_lw_id = match &area.lw_pre_aga {
        Some(version) => version.spriteset_id,
        None => area.lw.spriteset_id,
    };

    let spriteset_lw = spritesets[&spriteset_lw_id].clone();
    let spriteset_dw_id = dw.spriteset_id;
    let spriteset_dw = spritesets[&spriteset_dw_id].clone();

    if preserved_lw_sheets.is_empty() {
        if let Some(lw_v1) = &mut area.lw_pre_aga {
            lw_v1.spriteset_id = spriteset_dw_id;
        }
        area.lw.spriteset_id = spriteset_dw_id;
    } else if preserved_lw_sheets.len() == 4 {
        // If we are preserving all 4 spots, do nothing.
    } else {
        let spriteset_id = unused_spritesets
            .pop()
            .expect("Spritesets should have available slots");
        let spriteset = spritesets
            .get_mut(&spriteset_id)
            .expect(&format!("Spriteset {} should exist", &spriteset_id));

        if let Some(lw_v1) = &mut area.lw_pre_aga {
            lw_v1.spriteset_id = spriteset_id;
        }
        area.lw.spriteset_id = spriteset_id;

        for index in 0..4usize {
            if preserved_lw_sheets.contains(&index) {
                spriteset.sheets[index] = spriteset_lw.sheets[index];
            } else {
                spriteset.sheets[index] = spriteset_dw.sheets[index];
            }
        }
    }
    if preserved_dw_sheets.is_empty() {
        dw.spriteset_id = spriteset_lw_id;
    } else if preserved_dw_sheets.len() == 4 {
        // If we are preserving all 4 spots, do nothing.
    } else {
        // Free a spriteset up and reassign this version to that spriteset.
        let spriteset_id = unused_spritesets
            .pop()
            .expect("Spritesets should have available slots");
        let spriteset = spritesets
            .get_mut(&spriteset_id)
            .expect(&format!("Spriteset {} should exist", &spriteset_id));
        dw.spriteset_id = spriteset_id;

        for index in 0..4 {
            if preserved_dw_sheets.contains(&index) {
                spriteset.sheets[index] = spriteset_dw.sheets[index];
            } else {
                spriteset.sheets[index] = spriteset_lw.sheets[index];
            }
        }
    }
}

fn swap_spritesets(area: &mut OWRoom) {
    if area.dw.is_none() {
        // Skip inversion for areas that don't have both a light and dark world.
        // It is assumed that light world v2 exists if any light world exists
        return;
    }
    // Capture palette and spritesheet initial values upfront
    let light_spriteset_id = area.lw.spriteset_id;

    let dw = area.dw.as_mut().unwrap();
    let lw = &mut area.lw;

    let dark_spriteset_id = dw.spriteset_id;

    if let Some(lw_v1) = &mut area.lw_pre_aga {
        lw_v1.spriteset_id = dark_spriteset_id;
    }

    lw.spriteset_id = dark_spriteset_id;
    dw.spriteset_id = light_spriteset_id;
}

fn all_sheets_range() -> Vec<usize> {
    [0, 1, 2, 3].to_vec()
}

/// List of special areas that require preservation of some of their sheets.
/// If all indexes are preserved, the sprite sheet is left fully intact.
/// If a non-zero number of indexes less than max are provided, it creates a new spritesheet to
/// capture changes.
fn get_overworld_configuration(overworld_id: OWRoomId) -> Option<(Vec<usize>, Vec<usize>)> {
    match overworld_id {
        // Preserve Witch and zoras
        OWRoomId::x16_WITCHS_HUT => Some((vec![2], vec![2])),
        // This is only NPCs, so just skip inversion.
        OWRoomId::x28_KAKARIKO_VILLAGE_MAZE_RACE => Some((all_sheets_range(), all_sheets_range())),
        // Preserve frog black smith
        OWRoomId::x29_KAKARIKO_VILLAGE_LIBRARY => Some((vec![], all_sheets_range())),
        // Purple chest.
        OWRoomId::x22_SMITHY => Some((vec![], vec![0, 3])),
        // Only NPCs and creatures.
        OWRoomId::x2A_HAUNTED_GROVE => Some((all_sheets_range(), all_sheets_range())),
        // Dash item in v2 (dead tree).
        OWRoomId::x2_LUMBER_JACK_HOUSE => Some((vec![0, 2], vec![])),
        // Preserve catfish.
        OWRoomId::xF_ENTRANCE_TO_ZORAS_DOMAIN => Some((vec![], vec![2])),
        // Desert Statue and Tablet.
        OWRoomId::x30_DESERT_OF_MYSTERY => Some((vec![2], vec![])),
        // Rocks, Friends
        OWRoomId::x3_WEST_DEATH_MOUNTAIN => Some((vec![2], vec![2])),
        // Electric Barrier / Lighting Lock
        OWRoomId::x1B_HYRULE_CASTLE => Some((vec![3], vec![])),
        // Kiki during unlock animation.
        OWRoomId::x1E_EASTERN_PALACE => Some((vec![], vec![3])),
        // Average middle aged man.
        OWRoomId::x3A_PATH_BETWEEN_DESERT_OF_MYSTERY_AND_GREAT_SWAMP => Some((vec![3], vec![])),
        _ => None,
    }
}
