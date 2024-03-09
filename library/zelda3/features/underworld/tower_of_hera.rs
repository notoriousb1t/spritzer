use assembly::zelda3::Symbol;

use common::Patch;
use crate::zelda3::model::UWLayoutId;
use crate::zelda3::model::UWObject;
use crate::zelda3::model::UWObjectId;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::Z3Model;

pub(crate) fn relayout_hera_boss(model: &mut Z3Model) {
    let hera_scene = model
        .uw_scenes
        .get_mut(&UWRoomId::x07_TOWER_OF_HERA_MOLDORM_BOSS)
        .unwrap();
    // Remove everything from the room.
    for layer in hera_scene.layout.layers.iter_mut() {
        layer.clear();
    }
    // Add all required objects
    hera_scene.layout.layers[0].extend(get_hera_boss_objects());

    // Change the layout to divide the room into 4 rooms. This works for most bosses.
    hera_scene.layout.layout = UWLayoutId::X0_FourCorners;

    // Invalidate hera tower check when the layout is changed to 2x2.
    model.patches.push(Patch::of(
        Symbol::AncillaAdd_FallingPrize_hera_room_id.into(),
        0xFE,
    ));

    // Borrow the fairy room for the stairs so we can borrow the warp.
    let fairy_room = model
        .uw_scenes
        .get_mut(&UWRoomId::xA7_TOWER_OF_HERA_FAIRY_ROOM)
        .unwrap();

    // Remove the inner wall and warp tile.
    for layer in fairy_room.layout.layers.iter_mut() {
        layer.retain(|object| {
            object.id != UWObjectId::X0C0_CeilingLarge
                && object.id != UWObjectId::X062_WallTopEast
                && object.id != UWObjectId::X102_CornerTopConcave
                && object.id != UWObjectId::X103_CornerTopConcave
                && object.id != UWObjectId::X24A_WarpTile
        });
    }
    // Ensure 2 rows layout.
    fairy_room.layout.layout = UWLayoutId::X4_TwoRows;
    fairy_room.layout.layers[0].extend(get_hera_fairy_objects());

    // Redirect the stairs at the top to go to the bottom in a hidden room.
    let hera_fall_room_header = model
        .uw_headers
        .get_mut(&UWRoomId::x17_TOWER_OF_HERA_MOLDORM_FALL_ROOM)
        .unwrap();
    hera_fall_room_header.stairs0 = UWRoomId::xA7_TOWER_OF_HERA_FAIRY_ROOM;

    // Change the fairy room to warp to the boss room.
    let hera_fairy_room_header = model
        .uw_headers
        .get_mut(&UWRoomId::xA7_TOWER_OF_HERA_FAIRY_ROOM)
        .unwrap();
    hera_fairy_room_header.warp = UWRoomId::x07_TOWER_OF_HERA_MOLDORM_BOSS;
    // Kind of stupid, but the stairs down keep going to 0xC0.
    hera_fairy_room_header.stairs0 = UWRoomId::x17_TOWER_OF_HERA_MOLDORM_FALL_ROOM;
    hera_fairy_room_header.stairs1 = UWRoomId::x17_TOWER_OF_HERA_MOLDORM_FALL_ROOM;
    hera_fairy_room_header.stairs2 = UWRoomId::x17_TOWER_OF_HERA_MOLDORM_FALL_ROOM;
    hera_fairy_room_header.stairs3 = UWRoomId::x17_TOWER_OF_HERA_MOLDORM_FALL_ROOM;
}

fn get_hera_boss_objects() -> Vec<UWObject> {
    // Commented out pits for now because trinexx and kholdstare can't really have holes
    // until I figure out why you land under the lower room.
    vec![
        // Add an escape route from fairy side
        // UWObject::from_id(UWObjectId::X0A4_Pit, 0x11, 0x11, 0x1, 0x1),
        // Add pit in the north western corner.
        // UWObject::from_id(UWObjectId::X0A4_Pit, 0x26, 0x09, 0, 0),
        // UWObject::from_id(UWObjectId::X0A4_Pit, 0x35, 0x15, 0, 0x01),
    ]
}

fn get_hera_fairy_objects() -> Vec<UWObject> {
    vec![
        UWObject::from_id(UWObjectId::X0C0_CeilingLarge, 0x30, 0x03, 0x3, 0x2),
        UWObject::from_id(UWObjectId::X102_CornerTopConcave, 0x2C, 0x04, 0, 0),
        UWObject::from_id(UWObjectId::X062_WallTopEast, 0x2C, 0x08, 0x1, 0),
        UWObject::from_id(UWObjectId::X106_CornerTopConvex, 0x2C, 0x0F, 0x0, 0),
        UWObject::from_id(UWObjectId::X001_WallTopNorth, 0x30, 0x0F, 0x1, 0x1),
        UWObject::from_id(UWObjectId::X102_CornerTopConcave, 0x3A, 0x0F, 0, 0),
        UWObject::from_id(
            UWObjectId::X139_InterroomSpiralStairsDownTop,
            0x34,
            0x0F,
            0,
            0,
        ),
        // Add a pit to jump down to the boss.
        UWObject::from_id(UWObjectId::X0A4_Pit, 0x30, 0x16, 0x0, 0x0),
    ]
}
