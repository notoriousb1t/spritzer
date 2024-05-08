//! This module performs randomization of bosses.

use log;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::zelda3::features::dungeon::dungeon_conversion::convert_dungeon;
use crate::zelda3::model::get_arena_offsets;
use crate::zelda3::model::get_spritesheet_arrangements;
use crate::zelda3::model::get_vanilla_encounters;
use crate::zelda3::model::DungeonId;
use crate::zelda3::model::Encounter;
use crate::zelda3::model::ObjectInfo;
use crate::zelda3::model::Offsets;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteInfo;
use crate::zelda3::model::SpritesheetId;
use crate::zelda3::model::SpritesetId;
use crate::zelda3::model::UWFloorId;
use crate::zelda3::model::UWObject;
use crate::zelda3::model::UWObjectId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UWSprite;
use crate::zelda3::model::Z3Model;
use crate::zelda3::options::DEBUG_AGAHNIM1;
use crate::zelda3::options::DEBUG_ARMOS;
use crate::zelda3::options::DEBUG_ARRGHUS;
use crate::zelda3::options::DEBUG_BLIND;
use crate::zelda3::options::DEBUG_HELMASAUR;
use crate::zelda3::options::DEBUG_KHOLDSTARE;
use crate::zelda3::options::DEBUG_LANMOLAS;
use crate::zelda3::options::DEBUG_MOLDORM;
use crate::zelda3::options::DEBUG_MOTHULA;
use crate::zelda3::options::DEBUG_TRINEXX;
use crate::zelda3::options::DEBUG_VITREOUS;

/// Attempt to shuffle bosses.
pub(crate) fn apply_boss_shuffle(model: &mut Z3Model) {
    model.prepare_sprite_pool();
    let mut rng = model.create_rng();

    log::info!("Dungeon randomization:");

    if let Some(source_id) = check_for_debug_string(&model.debug_string) {
        debug_dungeons(model, &mut rng, source_id);
        return;
    }

    // Get the list of main boss rooms and filter out blocklisted ones.
    let original_dungeons = model
        .dungeons
        .keys()
        .filter(|it| can_shuffle_dungeon(it))
        .copied()
        .collect::<Vec<_>>();

    // Create a copy and shuffle the copy
    let mut shuffled_dungeons = original_dungeons.clone();
    shuffled_dungeons.shuffle(&mut rng);

    let len = original_dungeons.len();
    for i in 0..len {
        update_arena(model, &mut rng, shuffled_dungeons[i], original_dungeons[i]);
    }
}

pub(crate) fn can_shuffle_dungeon(group_id: &DungeonId) -> bool {
    match group_id {
        // Requires hookshot or cane of somaria.
        DungeonId::X0A_SwampPalace => false,
        // Requires firerod or bombos.
        DungeonId::X12_IcePalace => false,
        // Requires firerod and icerod
        DungeonId::X18_TurtleRock => false,
        // Requires investigation to see if he can be moved.
        DungeonId::X1A_GanonsTower => false,
        // Requires investigation to see if he can be moved.
        DungeonId::X08_AgahnimsTower => false,
        // Disable this for now since it is more important to get most of the other dungeons up and
        // running.
        _ => true,
    }
}

/// This is for debugging. If the seed has the boss name use that boss for every dungeon.
fn debug_dungeons(model: &mut Z3Model, rng: &mut StdRng, source_id: DungeonId) {
    let mut dungeon_list = model.dungeons.keys().copied().collect::<Vec<_>>();
    dungeon_list.sort();

    for target_id in dungeon_list.iter() {
        update_arena(model, rng, source_id, *target_id);
    }
}

fn update_arena(model: &mut Z3Model, rng: &mut StdRng, source_id: DungeonId, target_id: DungeonId) {
    if source_id == target_id {
        // Don't bother reconstructing if the target and source rooms are the same.
        log::debug!("  {} +", target_id);
        return;
    }

    let source_dungeon = model
        .dungeons
        .get(&source_id)
        .expect(&format!("Dungeon {} should exist", &source_id));
    let target_dungeon = model
        .dungeons
        .get(&target_id)
        .expect(&format!("Dungeon {} should exist", &target_id));
    if source_dungeon.boss.is_none() || target_dungeon.boss.is_none() {
        // We can only switch a boss if there is one.
        log::debug!("  {} »", target_id);
        return;
    }

    let source_boss = source_dungeon.boss.unwrap();
    let target_boss = target_dungeon.boss.unwrap();
    let target_room_offsets = get_arena_offsets(source_boss, target_boss);
    let encounter_list = get_vanilla_encounters(source_boss);
    let encounter = encounter_list.choose(rng).unwrap();

    convert_dungeon(model, source_id, target_id, encounter);
    remove_shells(model, target_boss);

    match source_boss {
        UWRoomId::x06_SWAMP_PALACE_ARRGHUS_BOSS => {
            // remove_blocks(model, target_boss, encounter, &target_room_offsets);
            remove_conveyers(model, target_boss, encounter, &target_room_offsets);
            // Remove the original water layers so new ones can be added in a larger configuration.
            remove_water_layers(model, target_boss, encounter, &target_room_offsets);
        }
        UWRoomId::xDE_ICE_PALACE_KHOLDSTARE_BOSS => {
            remove_conveyers(model, target_boss, encounter, &target_room_offsets);
            remove_water_layers(model, target_boss, encounter, &target_room_offsets);
        }
        UWRoomId::xC8_EASTERN_PALACE_ARMOS_KNIGHTS_BOSS => {
            remove_water_layers(model, target_boss, encounter, &target_room_offsets);
        }
        UWRoomId::x07_TOWER_OF_HERA_MOLDORM_BOSS => {
            remove_conveyers(model, target_boss, encounter, &target_room_offsets);
            remove_water_layers(model, target_boss, encounter, &target_room_offsets);
        }
        UWRoomId::x90_MISERY_MIRE_VITREOUS_BOSS => {
            remove_conveyers(model, target_boss, encounter, &target_room_offsets);
            remove_water_layers(model, target_boss, encounter, &target_room_offsets);
        }
        _ => {}
    }

    align_floors_if_necessary(model, target_boss, encounter);
    add_sprites(
        model,
        source_boss,
        target_boss,
        &encounter.sprites,
        &target_room_offsets,
    );
    isolate_spriteset(model, target_boss);
    add_objects(model, target_boss, &encounter.objects, &target_room_offsets);

    log::info!("  {} = {}", target_id, source_id);
}

/// Change the floors if the encounter requires it.
fn align_floors_if_necessary(model: &mut Z3Model, target_boss: UWRoomId, encounter: &Encounter) {
    if let Some(floor1) = encounter.floor1 {
        // Update the floor1 if specified. Update both places it is configured.
        let headers = model.uw_headers.get_mut(&target_boss).unwrap();
        headers.planes2 = UWFloorId::from_repr(floor1).unwrap();

        let scene = model.uw_scenes.get_mut(&target_boss).unwrap();
        scene.layout.floor1 = floor1;
    }
    if let Some(floor2) = encounter.floor2 {
        // Update the floor2 if specified. Update both places it is configured.
        let headers = model.uw_headers.get_mut(&target_boss).unwrap();
        headers.planes1 = UWFloorId::from_repr(floor2).unwrap();

        let scene = model.uw_scenes.get_mut(&target_boss).unwrap();
        scene.layout.floor2 = floor2;
    }
    if let Some(bg2_property) = encounter.bg2_property {
        let headers = model.uw_headers.get_mut(&target_boss).unwrap();
        headers.bg2_property = bg2_property;
    }
    if let Some(bgmove) = encounter.bgmove {
        let headers = model.uw_headers.get_mut(&target_boss).unwrap();
        headers.bgmove = bgmove;
    }
}

/// Update the spriteset of an underworld room to match the sprites present.
fn isolate_spriteset(model: &mut Z3Model, room_id: UWRoomId) {
    // Update the spriteset so it can render the selected sprites correctly.
    let original_spriteset_id = model.uw_headers[&room_id].spriteset_id;
    let new_spriteset_id = match spriteset_usage_count(model, original_spriteset_id) == 1 {
        true => original_spriteset_id,
        false => model.uw_spritesets_unused.pop().unwrap(),
    };
    let spriteset = model.spritesets.get_mut(&new_spriteset_id).unwrap();
    let room = model.uw_sprites.get(&room_id).unwrap();
    spriteset.sheets =
        get_spriteset_for_sprites(&room.sprites.iter().map(|sprite| sprite.id).collect());

    if let Some(headers) = model.uw_headers.get_mut(&room_id) {
        headers.spriteset_id = new_spriteset_id;
    }
}

fn remove_water_layers(
    model: &mut Z3Model,
    target_boss: UWRoomId,
    _encounter: &Encounter,
    _offsets: &Offsets,
) {
    let scene = model.uw_scenes.get_mut(&target_boss).unwrap();
    for layer in scene.layout.layers.iter_mut() {
        layer.retain(|object| !is_water(object.id));
    }
}

fn remove_shells(model: &mut Z3Model, target_boss: UWRoomId) {
    let scene = model.uw_scenes.get_mut(&target_boss).unwrap();
    for layer in scene.layout.layers.iter_mut() {
        layer.retain(|it| {
            it.id != UWObjectId::X215_KholdstaresShell && it.id != UWObjectId::X272_TrinexxsShell
        });
    }
}

fn remove_conveyers(
    model: &mut Z3Model,
    target_boss: UWRoomId,
    _encounter: &Encounter,
    _offsets: &Offsets,
) {
    let scene = model.uw_scenes.get_mut(&target_boss).unwrap();

    // Remove the conveyer floor for kholdstare. This comes down to a layering issue.
    // Both the transparency effect and the moving floors use the same
    // layering slot, so this just needs to be removed.
    for layer in scene.layout.layers.iter_mut() {
        layer.retain(|it| {
            it.id != UWObjectId::X0CA_ConveyorFloor && it.id != UWObjectId::X0C6_Layer2MaskLarge
        });
    }
}

fn is_water(object_id: UWObjectId) -> bool {
    matches!(
        object_id,
        UWObjectId::X0C8_WaterFloor
            | UWObjectId::X03F_WaterEdge
            | UWObjectId::X040_WaterEdge
            | UWObjectId::X041_WaterEdge
            | UWObjectId::X042_WaterEdge
            | UWObjectId::X043_WaterEdge
            | UWObjectId::X044_WaterEdge
            | UWObjectId::X045_WaterEdge
            | UWObjectId::X046_WaterEdge
            | UWObjectId::X079_WaterEdgeWest
            | UWObjectId::X07A_WaterEdgeEast
    )
}

/// Counts the number of usages of the provides spriteset.
fn spriteset_usage_count(model: &Z3Model, spriteset_id: SpritesetId) -> usize {
    // TODO: move this function somewhere more general and use it for other spriteset swaps.
    let overworld_usages: usize = model
        .ow_rooms
        .values()
        .map(|area| {
            area.states()
                .iter()
                .filter(|version| version.spriteset_id == spriteset_id)
                .count()
        })
        .sum();
    let underworld_usages: usize = model
        .uw_headers
        .values()
        .filter(|area| area.spriteset_id == spriteset_id)
        .count();
    overworld_usages + underworld_usages
}

// Returns the first valid combination fo spritesheets that satisfies the SpriteId list.
fn get_spriteset_for_sprites(sprite_ids: &Vec<SpriteId>) -> [SpritesheetId; 4] {
    // TODO: move this function somewhere more general and use it for other spriteset swaps.
    let mut spritesheet = [SpritesheetId::None; 4];

    for sprite_id in sprite_ids {
        // This is a bit flawed, since sprites can have multiple requirements.
        // TODO: refactor this so it instead generates all valid unions.
        let requirement_list: Vec<[SpritesheetId; 4]> = get_spritesheet_arrangements(sprite_id);
        let requirement = requirement_list
            .first()
            .unwrap_or(&[SpritesheetId::None; 4]);
        for i in 0..4 {
            if requirement[i] != SpritesheetId::None {
                spritesheet[i] = requirement[i];
            }
        }
    }

    spritesheet
}

fn add_sprites(
    model: &mut Z3Model,
    source_boss: UWRoomId,
    target_boss: UWRoomId,
    sprite_infos: &[SpriteInfo],
    offsets: &Offsets,
) {
    let mut sprite_infos = sprite_infos.to_vec();

    // Add any room specific sprites that are independent of the boss.
    if target_boss == UWRoomId::x29_SKULL_WOODS_MOTHULA_BOSS {
        match source_boss {
            UWRoomId::xDE_ICE_PALACE_KHOLDSTARE_BOSS => {}
            _ => {
                // Add movable floor so the battle is more similar to the original.
                sprite_infos.push(SpriteInfo::new(SpriteId::x107_MOVING_FLOOR, 0, 3));
            }
        }
    }

    // Get a mutable reference to the room and start generating sprites.
    let room = model.uw_sprites.get_mut(&target_boss).unwrap();
    room.sprites = sprite_infos
        .iter()
        .map(|sprite_info| create_sprite(sprite_info, offsets))
        .collect::<Vec<_>>();
}

fn add_objects(
    model: &mut Z3Model,
    target_boss: UWRoomId,
    object_infos: &[ObjectInfo],
    offsets: &Offsets,
) {
    // Create all new objects and put them into their layers.
    let mut layers: Vec<Vec<UWObject>> = vec![vec![], vec![], vec![]];
    for info in object_infos.iter() {
        let mut new_object = info.object.clone();
        // Arena offsets are calculated by sprite x,y are twice as large per
        // increment compared to the precision of room objects.
        new_object.x += offsets.x * 2;
        new_object.y += offsets.y * 2;
        layers[info.layer as usize].push(new_object);
    }

    // Update the scene such that the new objects are inserted into the existing layers
    // with lower precedence than other objects.
    let scene = model.uw_scenes.get_mut(&target_boss).unwrap();
    for (index, layer) in layers.iter().enumerate() {
        for object in layer.iter().rev() {
            scene.layout.layers[index].insert(0, object.to_owned());
        }
    }
}

fn create_sprite(sprite_info: &SpriteInfo, offsets: &Offsets) -> UWSprite {
    UWSprite {
        id: sprite_info.id,
        x_pos: sprite_info.x + offsets.x,
        y_pos: sprite_info.y + offsets.y,
        lower_layer: offsets.lower_layer,
        item: None,
        aux0: 0,
        aux1: 0,
        distance_from_midpoint: 0,
    }
}

fn check_for_debug_string(seed: &str) -> Option<DungeonId> {
    for (word, dungeon_id) in [
        (DEBUG_ARMOS, DungeonId::X04_EasternPalace),
        (DEBUG_LANMOLAS, DungeonId::X06_DesertPalace),
        (DEBUG_MOLDORM, DungeonId::X14_TowerOfHera),
        (DEBUG_HELMASAUR, DungeonId::X0C_PalaceOfDarkness),
        (DEBUG_ARRGHUS, DungeonId::X0A_SwampPalace),
        (DEBUG_MOTHULA, DungeonId::X10_SkullWoods),
        (DEBUG_BLIND, DungeonId::X16_ThievesTown),
        (DEBUG_KHOLDSTARE, DungeonId::X12_IcePalace),
        (DEBUG_VITREOUS, DungeonId::X0E_MiseryMire),
        (DEBUG_TRINEXX, DungeonId::X18_TurtleRock),
        (DEBUG_AGAHNIM1, DungeonId::X08_AgahnimsTower),
    ] {
        if seed.contains(word) {
            return Some(dungeon_id);
        }
    }
    None
}
