use crate::zelda3::model::SpriteId;
use crate::zelda3::model::UWFloorId;
use crate::zelda3::model::UWObject;
use crate::zelda3::model::UWObjectId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::Z3Model;

pub(crate) fn relayout_swamp_map_chest_room(model: &mut Z3Model) {
    // Without these, the stair case replacement breaks water fill logic in previous room.
    let scene = model
        .uw_scenes
        .get_mut(&UWRoomId::x37_SWAMP_PALACE_MAP_CHEST_WATER_FILL_ROOM)
        .unwrap();
    let middle_layer = scene.layout.layers.get_mut(0).unwrap();
    middle_layer.push(UWObject::from_id(UWObjectId::X134_Block, 0x10, 0x30, 0, 0));
    middle_layer.push(UWObject::from_id(UWObjectId::X134_Block, 0x10, 0x32, 0, 0));
}

pub(crate) fn relayout_swamp_entrance_room(model: &mut Z3Model) {
    // Without these, the stair case replacement breaks required Magic Mirror.
    let scene = model
        .uw_scenes
        .get_mut(&UWRoomId::x28_SWAMP_PALACE_ENTRANCE_ROOM)
        .unwrap();
    let middle_layer = scene.layout.layers.get_mut(0).unwrap();
    middle_layer.push(UWObject::from_id(UWObjectId::X134_Block, 0x18, 0x2A, 0, 0));
    middle_layer.push(UWObject::from_id(UWObjectId::X134_Block, 0x18, 0x2C, 0, 0));
}

pub(crate) fn relayout_swamp_water_treadmill_room(model: &mut Z3Model) {
    // Change to a flying sprite that is loaded since water tektites only work when put in
    // water areas with edges.
    let scene = model
        .uw_scenes
        .get_mut(&UWRoomId::x16_SWAMP_PALACE_SWIMMING_TREADMILL)
        .unwrap();
    let sprites = model
        .uw_sprites
        .get_mut(&UWRoomId::x16_SWAMP_PALACE_SWIMMING_TREADMILL)
        .unwrap();

    sprites
        .sprites
        .retain(|sprite| sprite.id != SpriteId::x81_WATER_TEKTITE);

    // Drain water and replace with regular stairs.
    for index in 0..3 {
        let layer = scene.layout.layers.get_mut(index).unwrap();
        layer.retain(|object| {
            object.id != UWObjectId::X0E7_HeavyCurrentWater
                && object.id != UWObjectId::X0D9_Layer2SwimMask
                && object.id != UWObjectId::X0C5_Floor3
        });
        for object in layer.iter_mut() {
            if object.id == UWObjectId::X135_WaterLadderNorth {
                object.id = UWObjectId::X132_IntraroomStairsNorthMergedLayers;
            }
        }
    }

    // Fix layer issues caused by removing water.
    let header = model
        .uw_headers
        .get_mut(&UWRoomId::x16_SWAMP_PALACE_SWIMMING_TREADMILL)
        .unwrap();
    header.planes2 = UWFloorId::x00_GROUND_NEUTRAL;
    header.planes1 = UWFloorId::x00_GROUND_NEUTRAL;
    header.bg2_property = 0;
}

pub(crate) fn relayout_swamp_hidden_chest_room(model: &mut Z3Model) {
    // Drain water and replace with regular stairs.
    let scene = model
        .uw_scenes
        .get_mut(&UWRoomId::x66_SWAMP_PALACE_HIDDEN_CHEST_HIDDEN_DOOR_ROOM)
        .unwrap();
    for layer in scene.layout.layers.iter_mut() {
        for object in layer.iter_mut() {
            if object.id == UWObjectId::X202_WaterfallFaceLong {
                object.id = UWObjectId::X031_Nothing;
            }
        }
    }
}
