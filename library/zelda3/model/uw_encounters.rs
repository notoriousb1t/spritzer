use super::dungeon::DungeonId;
use super::dungeon_blockset_id::UWBlocksetId;
use super::palette_id::PaletteId;
use super::sprite_id::SpriteId;
use super::uw_object_id::UWObjectId;
use super::uw_room_id::UWRoomId;
use super::{RoomBackground, RoomCollision, RoomEffect, UWFloorId};
use crate::zelda3::model::UWObject;

pub(crate) struct Encounter {
    #[allow(dead_code)]
    pub dungeon_id: Option<DungeonId>,
    pub sprites: Vec<SpriteInfo>,
    pub objects: Vec<ObjectInfo>,
    pub blockset_id: Option<UWBlocksetId>,
    pub palette_id: Option<PaletteId>,
    pub bg2: Option<RoomBackground>,
    pub effect: Option<RoomEffect>,
    pub collision: Option<RoomCollision>,
    pub light: Option<bool>,
    pub floor1: Option<UWFloorId>,
    pub floor2: Option<UWFloorId>,
}

impl Encounter {
    pub(crate) fn default() -> Encounter {
        Encounter {
            dungeon_id: None,
            sprites: vec![],
            objects: vec![],
            blockset_id: None,
            palette_id: None,
            bg2: None,
            effect: None,
            collision: None,
            light: None,
            floor1: None,
            floor2: None,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct SpriteInfo {
    pub id: SpriteId,
    pub x: u8,
    pub y: u8,
}

impl SpriteInfo {
    pub(crate) fn new(id: SpriteId, x: u8, y: u8) -> Self {
        SpriteInfo { id, x, y }
    }
}

pub(crate) struct ObjectInfo {
    pub layer: u8,
    pub object: UWObject,
}

/// Returns a configuration resembling the vanilla game. This function returns
/// a vector of configurations so that more difficult versions can be built and
/// chosen at random.
pub(crate) fn get_vanilla_encounters(room_id: UWRoomId) -> Vec<Encounter> {
    match room_id {
        UWRoomId::x1C_GANON_S_TOWER_ICE_ARMOS => vec![Encounter {
            sprites: vec![
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0x4, 0x5),
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0x7, 0x5),
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0xA, 0x5),
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0xA, 0x8),
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0x7, 0x8),
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0x4, 0x8),
                SpriteInfo::new(SpriteId::x119_ARMOS_KNIGHTS_TRIGGER, 0x7, 0x9),
            ],
            blockset_id: Some(UWBlocksetId::xE_DESERT_PALACE),
            palette_id: Some(PaletteId::x1A_GANONS_TOWER),
            ..Encounter::default()
        }],
        UWRoomId::x6C_GANON_S_TOWER_LANMOLAS_ROOM => vec![Encounter {
            sprites: vec![
                SpriteInfo::new(SpriteId::x54_LANMOLAS, 0x6, 0x7),
                SpriteInfo::new(SpriteId::x54_LANMOLAS, 0x7, 0x7),
                SpriteInfo::new(SpriteId::x54_LANMOLAS, 0x5, 0x9),
                SpriteInfo::new(SpriteId::xC5_MEDUSA, 0x1, 0xB),
            ],
            blockset_id: Some(UWBlocksetId::xE_DESERT_PALACE),
            palette_id: Some(PaletteId::x24_GANONS_TOWER),
            ..Encounter::default()
        }],
        UWRoomId::x4D_GANON_S_TOWER_MOLDORM_ROOM => vec![Encounter {
            sprites: vec![SpriteInfo::new(SpriteId::x9_MOLDORM, 0x6, 0x7)],
            blockset_id: Some(UWBlocksetId::xE_DESERT_PALACE),
            palette_id: Some(PaletteId::x1A_GANONS_TOWER),
            ..Encounter::default()
        }],
        UWRoomId::xC8_EASTERN_PALACE_ARMOS_KNIGHTS_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X04_EasternPalace),
            sprites: vec![
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0x7, 0x6),
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0x7, 0x6),
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0x7, 0x6),
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0x7, 0x6),
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0x7, 0x6),
                SpriteInfo::new(SpriteId::x53_ARMOS_KNIGHT, 0x7, 0x6),
                SpriteInfo::new(SpriteId::x119_ARMOS_KNIGHTS_TRIGGER, 0x7, 0x8),
            ],
            blockset_id: Some(UWBlocksetId::x5_EASTERN_PALACE),
            palette_id: Some(PaletteId::x0B_MISERY_MIRE),
            ..Encounter::default()
        }],
        UWRoomId::x33_DESERT_PALACE_LANMOLAS_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X06_DesertPalace),
            sprites: vec![
                SpriteInfo::new(SpriteId::x54_LANMOLAS, 0x6, 0x7),
                SpriteInfo::new(SpriteId::x54_LANMOLAS, 0x9, 0x7),
                SpriteInfo::new(SpriteId::x54_LANMOLAS, 0x7, 0x9),
            ],
            blockset_id: Some(UWBlocksetId::x5_EASTERN_PALACE),
            palette_id: Some(PaletteId::x04_ICE_DUNGEON),
            ..Encounter::default()
        }],
        UWRoomId::x07_TOWER_OF_HERA_MOLDORM_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X14_TowerOfHera),
            sprites: vec![SpriteInfo::new(SpriteId::x9_MOLDORM, 0x6, 0x7)],
            blockset_id: Some(UWBlocksetId::x5_EASTERN_PALACE),
            palette_id: Some(PaletteId::x06_TOWER_OF_HERA),
            ..Encounter::default()
        }],
        UWRoomId::x20_AGAHNIM_S_TOWER_AGAHNIM_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X08_AgahnimsTower),
            sprites: vec![SpriteInfo::new(SpriteId::x7A_AGAHNIM, 0x8, 0x5)],
            blockset_id: Some(UWBlocksetId::x2_AGAHNIM),
            palette_id: Some(PaletteId::x0C_AGAHNIMS_TOWER),
            ..Encounter::default()
        }],
        UWRoomId::x5A_PALACE_OF_DARKNESS_HELMASAUR_KING_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X0C_PalaceOfDarkness),
            sprites: vec![SpriteInfo::new(SpriteId::x92_KING_HELMASAUR, 0x7, 0x6)],
            blockset_id: Some(UWBlocksetId::x7_TOWER_HERA),
            palette_id: Some(PaletteId::x10_PALACE_OF_DARKNESS_HELMASAUR),
            ..Encounter::default()
        }],
        UWRoomId::x06_SWAMP_PALACE_ARRGHUS_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X0A_SwampPalace),
            sprites: vec![
                SpriteInfo::new(SpriteId::x8C_ARRGHUS, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
                SpriteInfo::new(SpriteId::x8D_ARRGHUS_SPAWN, 0x4, 0x7),
            ],
            objects: get_water_floor_with_border(),
            blockset_id: Some(UWBlocksetId::x8_SWAMP_PALACE),
            palette_id: Some(PaletteId::x08_SWAMP_PALACE),
            ..Encounter::default()
        }],
        UWRoomId::x29_SKULL_WOODS_MOTHULA_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X10_SkullWoods),
            sprites: vec![SpriteInfo::new(SpriteId::x88_MOTHULA, 0x6, 0x5)],
            blockset_id: Some(UWBlocksetId::x9_SKULL_WOODS),
            palette_id: Some(PaletteId::x0E_CHURCH_SANCTUARY),
            ..Encounter::default()
        }],
        UWRoomId::xAC_THIEVES_TOWN_BLIND_THE_THIEF_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X16_ThievesTown),
            sprites: vec![SpriteInfo::new(SpriteId::xCE_BLIND, 0x9, 0x5)],
            blockset_id: Some(UWBlocksetId::xA_THIEVES_TOWN),
            palette_id: Some(PaletteId::x17_THIEVES_TOWN),
            ..Encounter::default()
        }],
        UWRoomId::xDE_ICE_PALACE_KHOLDSTARE_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X12_IcePalace),
            sprites: vec![
                SpriteInfo::new(SpriteId::xA3_KHOLDSTARES_SHELL, 0x7, 0x5),
                SpriteInfo::new(SpriteId::xA4_FALLING_ICE, 0x7, 0x5),
                SpriteInfo::new(SpriteId::xA2_KHOLDSTARE, 0x7, 0x5),
            ],
            objects: get_ice_floor()
                .into_iter()
                .chain(get_kholdstare_shell().into_iter())
                .collect::<Vec<_>>(),
            bg2: Some(RoomBackground::Translucent),
            collision: Some(RoomCollision::One),
            effect: Some(RoomEffect::Nothing),
            blockset_id: Some(UWBlocksetId::xB_ICE_PALACE),
            palette_id: Some(PaletteId::x14_ICE_PALACE_KHOLDSTARE),
            floor1: Some(UWFloorId::x04_GROUND_ACCENT),
            floor2: Some(UWFloorId::x0E_DARKNESS),
            ..Encounter::default()
        }],
        UWRoomId::x90_MISERY_MIRE_VITREOUS_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X0E_MiseryMire),
            sprites: vec![SpriteInfo::new(SpriteId::xBD_VITREOUS, 0x7, 0x5)],
            blockset_id: Some(UWBlocksetId::xC_MISERY_MIRE),
            palette_id: Some(PaletteId::x12_MISERY_MIRE_VITREUS),
            ..Encounter::default()
        }],
        UWRoomId::xA4_TURTLE_ROCK_TRINEXX_BOSS => vec![Encounter {
            dungeon_id: Some(DungeonId::X18_TurtleRock),
            sprites: vec![
                SpriteInfo::new(SpriteId::xCB_TRINEXX_ROCK, 0x7, 0x7),
                SpriteInfo::new(SpriteId::xCC_TRINEXX_FIRE, 0x7, 0x7),
                SpriteInfo::new(SpriteId::xCD_TRINEXX_ICE, 0x7, 0x7),
            ],
            objects: vec![ObjectInfo {
                layer: 1,
                object: UWObject::from_id(UWObjectId::X272_TrinexxsShell, 0xB, 0xA, 0x0, 0x0),
            }],
            bg2: Some(RoomBackground::OnTop),
            collision: Some(RoomCollision::One),
            effect: Some(RoomEffect::Four),
            blockset_id: Some(UWBlocksetId::xD_TURTLE_ROCK),
            palette_id: Some(PaletteId::x19_TURTLE_ROCK_TRINEXX),
            floor1: Some(UWFloorId::x01_GROUND_PRIMARY),
            floor2: Some(UWFloorId::x0E_DARKNESS),
            ..Encounter::default()
        }],
        UWRoomId::x0D_GANON_S_TOWER_AGAHNIM2_BOSS => vec![Encounter {
            sprites: vec![SpriteInfo::new(SpriteId::x7A_AGAHNIM, 0x8, 0x5)],
            blockset_id: Some(UWBlocksetId::xE_DESERT_PALACE),
            palette_id: Some(PaletteId::x1B_GANONS_TOWER),
            ..Encounter::default()
        }],
        _ => {
            panic!("Unsupported boss {}", room_id);
        }
    }
}

fn get_kholdstare_shell() -> Vec<ObjectInfo> {
    vec![ObjectInfo {
        layer: 1,
        object: UWObject::from_id(UWObjectId::X215_KholdstaresShell, 0xB, 0x8, 0x0, 0x0),
    }]
}

/// Gets the objects to make a standard water boss floor.
fn get_ice_floor() -> Vec<ObjectInfo> {
    vec![
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X0D1_IcyFloorA, 0x04, 0x06, 0x3, 0x3),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X0D1_IcyFloorA, 0x14, 0x06, 0x1, 0x3),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X0D1_IcyFloorA, 0x04, 0x14, 0x2, 0x1),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X0D1_IcyFloorA, 0x0C, 0x12, 0x1, 0x1),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X0D1_IcyFloorA, 0x10, 0x14, 0x2, 0x1),
        },
    ]
}

/// Gets the objects to make a standard water boss floor.
fn get_water_floor_with_border() -> Vec<ObjectInfo> {
    vec![
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X0C8_WaterFloor, 0x04, 0x06, 0x3, 0x3),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X0C8_WaterFloor, 0x14, 0x06, 0x1, 0x3),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X0C8_WaterFloor, 0x04, 0x13, 0x2, 0x1),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X0C8_WaterFloor, 0x0C, 0x11, 0x1, 0x1),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X0C8_WaterFloor, 0x10, 0x13, 0x2, 0x1),
        },
        // Water edges
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X03F_WaterEdge, 0x04, 0x06, 0x3, 0x3),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X03F_WaterEdge, 0x12, 0x06, 0x1, 0x3),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X079_WaterEdgeWest, 0x04, 0x07, 0x3, 0x3),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X079_WaterEdgeWest, 0x04, 0x16, 0x1, 0x0),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X07A_WaterEdgeEast, 0x1B, 0x07, 0x3, 0x3),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X07A_WaterEdgeEast, 0x1B, 0x16, 0x1, 0x0),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X040_WaterEdge, 0x04, 0x1B, 0x1, 0x3),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X041_WaterEdge, 0x0D, 0x1A, 0x0, 0x3),
        },
        ObjectInfo {
            layer: 0,
            object: UWObject::from_id(UWObjectId::X040_WaterEdge, 0x12, 0x1B, 0x1, 0x3),
        },
    ]
}
