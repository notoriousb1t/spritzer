use std::fmt::Display;

use strum_macros::FromRepr;

use super::uw_room_id::UWRoomId;

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, FromRepr)]
#[allow(dead_code, non_camel_case_types)]
pub(crate) enum DungeonId {
    X00_Sewers = 0x00,
    X02_HyruleCastle = 0x02,
    X04_EasternPalace = 0x04,
    X06_DesertPalace = 0x06,
    X08_AgahnimsTower = 0x08,
    X0A_SwampPalace = 0x0A,
    X0C_PalaceOfDarkness = 0x0C,
    X0E_MiseryMire = 0x0E,
    X10_SkullWoods = 0x10,
    X12_IcePalace = 0x12,
    X14_TowerOfHera = 0x14,
    X16_ThievesTown = 0x16,
    X18_TurtleRock = 0x18,
    X1A_GanonsTower = 0x1A,
    XFF_Cave = 0xFF,
}

impl Display for DungeonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DungeonId::X00_Sewers => write!(f, "Sewers"),
            DungeonId::X02_HyruleCastle => write!(f, "Hyyrule Castle"),
            DungeonId::X04_EasternPalace => write!(f, "Eastern Palace"),
            DungeonId::X06_DesertPalace => write!(f, "Desert Palace"),
            DungeonId::X08_AgahnimsTower => write!(f, "Agahimn's Tower"),
            DungeonId::X0A_SwampPalace => write!(f, "Swamp Palace"),
            DungeonId::X0C_PalaceOfDarkness => write!(f, "Palace of Darkness"),
            DungeonId::X0E_MiseryMire => write!(f, "Misery Mire"),
            DungeonId::X10_SkullWoods => write!(f, "Skull Woods"),
            DungeonId::X12_IcePalace => write!(f, "Ice Palace"),
            DungeonId::X14_TowerOfHera => write!(f, "Tower of Hera"),
            DungeonId::X16_ThievesTown => write!(f, "Thieves' Town"),
            DungeonId::X18_TurtleRock => write!(f, "Turtle Rock"),
            DungeonId::X1A_GanonsTower => write!(f, "Ganon's Tower"),
            DungeonId::XFF_Cave => write!(f, "Cave"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Dungeon {
    pub dungeon_id: DungeonId,
    pub boss: Option<UWRoomId>,
    #[allow(dead_code)]
    pub minibosses: Vec<UWRoomId>,
    pub rooms: Vec<UWRoomId>,
}
