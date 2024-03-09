use super::eastern_palace::prepare_eastern_palace;
use super::swamp_palace::relayout_swamp_entrance_room;
use super::swamp_palace::relayout_swamp_hidden_chest_room;
use super::swamp_palace::relayout_swamp_map_chest_room;
use super::swamp_palace::relayout_swamp_water_treadmill_room;
use super::thieves_town::prepare_thieves_town_boss;
use super::tower_of_hera::relayout_hera_boss;
use common::Patch;
use crate::zelda3::model::get_arena_position;
use crate::zelda3::model::ArenaPosition;
use crate::zelda3::model::DungeonId;
use crate::zelda3::model::Encounter;
use crate::zelda3::model::UWBlocksetId;
use crate::zelda3::model::UWObjectId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::Z3Model;

/// Update grouped rooms to match the blockset and palette. Blocksets and palettes don't seem
/// to always update in game when changing rooms consistently, and this also makes the dungeon
/// theming more meaningful.
pub(crate) fn convert_dungeon(
    model: &mut Z3Model,
    source_id: DungeonId,
    target_id: DungeonId,
    encounter: &Encounter,
) {
    if source_id == target_id {
        // Don't do anything if the dungeons are the same.
        return;
    }

    for entrance in model.uw_entrances.values_mut() {
        if entrance.dungeon_id == target_id {
            if let Some(blockset_id) = encounter.blockset_id {
                entrance.blockset_id = blockset_id;
            }
        }
    }

    let target_dungeon = model.dungeons.get(&target_id).unwrap();
    for target_room_id in target_dungeon.rooms.iter() {
        if let Some(headers) = model.uw_headers.get_mut(target_room_id) {
            if let Some(blockset_id) = encounter.blockset_id {
                if can_change_blockset(target_room_id, blockset_id) {
                    headers.blockset_id = blockset_id;
                }
            }
            if let Some(palette_id) = encounter.palette_id {
                headers.palette_id = palette_id;
            }
        }
    }

    let source_dungeon = model.dungeons.get(&source_id).unwrap();
    let source_boss = source_dungeon.boss.unwrap();

    // Remove objects that are incompatible.
    let rooms = target_dungeon.rooms.to_vec();
    for target_room_id in rooms.iter() {
        add_room_specific_logic_fixes(model, source_boss, *target_room_id);
    }

    for target_room_id in rooms.iter() {
        if let Some(scene) = model.uw_scenes.get_mut(target_room_id) {
            let layer_count = scene.layout.layers.len();
            for index in 0..layer_count {
                let layer = scene.layout.layers.get_mut(index).unwrap();

                // Attempt to perform hand-picked swaps
                for object in layer.iter_mut() {
                    let new_id = swap_tileset_objects(source_id, target_id, object.id);
                    if (object.width > 0 || object.height > 0) && new_id != object.id {
                        // If the new and old object are not the same and the size is greater than a
                        // standard unit, reset the size to 0 (which is just
                        // the smallest size)
                        object.width = 0;
                        object.height = 0;
                    }

                    object.id = new_id;
                }

                // Remove anything known to be completely incompatible.
                layer.retain(|it| can_retain_object(source_id, target_id, it.id));
            }
        }
    }

    //
}

/// Add fixes for specific rooms where logic is broken by swapping objects.
fn add_room_specific_logic_fixes(model: &mut Z3Model, source_id: UWRoomId, target_id: UWRoomId) {
    match target_id {
        UWRoomId::xA9_EASTERN_PALACE_BIG_CHEST_ROOM => {
            prepare_eastern_palace(model, source_id, target_id);
        }
        UWRoomId::xB8_EASTERN_PALACE_BIG_KEY_ROOM => {
            prepare_eastern_palace(model, source_id, target_id);
        }
        UWRoomId::xAC_THIEVES_TOWN_BLIND_THE_THIEF_BOSS => {
            prepare_thieves_town_boss(model);
        }
        UWRoomId::x07_TOWER_OF_HERA_MOLDORM_BOSS => {
            if get_arena_position(source_id, target_id) != ArenaPosition::Center {
                relayout_hera_boss(model);
            }
        }
        UWRoomId::x36_SWAMP_PALACE_BIG_CHEST_ROOM => {
            // For whatever reason, the code prevents the big key chest room from changing palettes
            // when transitioning away. This jumps past all checks to preserve palette changes.
            // TODO: Verify.
            model
                .patches
                .push(Patch::update_local_pointer(0x2_8A3D, 0x2_8A59))
        }
        UWRoomId::x37_SWAMP_PALACE_MAP_CHEST_WATER_FILL_ROOM => {
            relayout_swamp_map_chest_room(model);
        }
        UWRoomId::x28_SWAMP_PALACE_ENTRANCE_ROOM => {
            relayout_swamp_entrance_room(model);
        }
        UWRoomId::x16_SWAMP_PALACE_SWIMMING_TREADMILL => {
            relayout_swamp_water_treadmill_room(model);
        }
        UWRoomId::x66_SWAMP_PALACE_HIDDEN_CHEST_HIDDEN_DOOR_ROOM => {
            relayout_swamp_hidden_chest_room(model);
        }
        _ => {}
    }
}

/// This function returns true if the object is compatible with the blockset change.
/// This is used to remove objects that don't generally work across blocksets.
fn can_retain_object(_source_id: DungeonId, _target_id: DungeonId, object_id: UWObjectId) -> bool {
    !matches!(object_id, UWObjectId::X031_Nothing)
}

fn swap_tileset_objects(
    source_id: DungeonId,
    target_id: DungeonId,
    object_id: UWObjectId,
) -> UWObjectId {
    // TODO: fix decorative objects in desert for skull woods blockset.

    // Perform unconditional swaps if nothing else matches.
    let object_id = match object_id {
        // Replace cannons (which mostly occur in EP) with decors because the
        // graphics are glitched in other blocksets.
        UWObjectId::X052_CannonHoleASouth => UWObjectId::X03B_WallDecorsSouth,
        UWObjectId::X051_CannonHoleANorth => UWObjectId::X03A_WallDecorsNorth,
        UWObjectId::X085_CannonHoleWest => UWObjectId::X076_WallDecorsWest,
        UWObjectId::X086_CannonHoleEast => UWObjectId::X077_WallDecorsEast,
        // These are just signals in POD but have collisions for non-POD blocksets. Just remove
        // them if a dungeon swap happens.
        // Can't proceed in POD.
        UWObjectId::X27C_ArrowTile => UWObjectId::X031_Nothing,
        UWObjectId::X27D_ArrowTile => UWObjectId::X031_Nothing,
        UWObjectId::X27E_ArrowTile => UWObjectId::X031_Nothing,
        // Grates work almost exclusively on Swamp and it doesn't look bad to remove otherwise.
        UWObjectId::X26C_WaterGrateNorth => UWObjectId::X031_Nothing,
        UWObjectId::X26D_WaterGrateSouth => UWObjectId::X031_Nothing,
        UWObjectId::X26E_WaterGrateWest => UWObjectId::X031_Nothing,
        UWObjectId::X26F_WaterGrateEast => UWObjectId::X031_Nothing,
        _ => object_id,
    };

    // Swap anything specific to the source first.
    // Continue applying swap logic until no more swaps can be made.
    let object_id = match target_id {
        DungeonId::X04_EasternPalace => match object_id {
            // TODO: handle eastern palace specifically and create platforms out of stairs
            // because just removing them is super boring.
            UWObjectId::X0DC_ChestPlatformShort => UWObjectId::X031_Nothing,
            // TODO: handle eastern palace specifically and create platforms out of stairs
            // because just removing them is super boring.
            UWObjectId::X0C1_ChestPlatformTall => UWObjectId::X031_Nothing,
            UWObjectId::X021_PlatformStairs => UWObjectId::X031_Nothing,
            _ => object_id,
        },
        DungeonId::X0A_SwampPalace => match object_id {
            // Swap ladders with stairs. (Note: this has to be compensated with blocks)
            UWObjectId::X135_WaterLadderNorth => UWObjectId::X133_IntraroomStairsNorthSwimLayer,
            UWObjectId::X136_WaterLadderSouth => UWObjectId::X233_IntraroomStairsSouthSwimLayer,
            _ => object_id,
        },
        DungeonId::X0E_MiseryMire => match object_id {
            // Replace the misery mire bridge with something that will load in.
            UWObjectId::X0E8_Floor10 => UWObjectId::X0C5_Floor3,
            // Remove these during swaps. This is either never executed for misery mire natural and
            // This will only be removed for misery mire targets.
            UWObjectId::X262_VitreousGooGraphics => UWObjectId::X031_Nothing,
            UWObjectId::X27B_VitreousGooDamage => UWObjectId::X031_Nothing,
            _ => object_id,
        },
        _ => object_id,
    };

    match source_id {
        DungeonId::X10_SkullWoods => match object_id {
            UWObjectId::X090_WallFlairWest => UWObjectId::X031_Nothing,
            UWObjectId::X091_WallFlairEast => UWObjectId::X031_Nothing,
            UWObjectId::X0B6_WallFlairNorth => UWObjectId::X031_Nothing,
            UWObjectId::X0B7_WallFlairSouth => UWObjectId::X031_Nothing,
            _ => object_id,
        },
        DungeonId::X12_IcePalace => match object_id {
            // Replace all water artifacts with ice ones.
            UWObjectId::X0C8_WaterFloor => UWObjectId::X0D1_IcyFloorA,
            UWObjectId::X200_WaterfallFaceEmpty => UWObjectId::X055_WallTorchesNorth,
            UWObjectId::X201_WaterfallFaceShort => UWObjectId::X031_Nothing,
            UWObjectId::X202_WaterfallFaceLong => UWObjectId::X031_Nothing,
            _ => object_id,
        },

        DungeonId::X0A_SwampPalace => match object_id {
            // Replace all water artifacts with ice ones.
            UWObjectId::X0D1_IcyFloorA => UWObjectId::X0C8_WaterFloor,
            UWObjectId::X0D2_IcyFloorB => UWObjectId::X0C8_WaterFloor,
            _ => object_id,
        },
        _ => object_id,
    }
}

/// Turns off problematic blockset changes for rooms where it impacts functionality.
pub(crate) fn can_change_blockset(room_id: &UWRoomId, _blockset_id: UWBlocksetId) -> bool {
    !matches!(
        room_id,
        UWRoomId::x30_AGAHNIM_S_TOWER_MAIDEN_SACRIFICE_CHAMBER
            | UWRoomId::x20_AGAHNIM_S_TOWER_AGAHNIM_BOSS
    )
}
