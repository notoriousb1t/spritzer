use strum_macros::Display;
use strum_macros::FromRepr;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Hash, FromRepr)]
#[allow(non_camel_case_types)]
pub(crate) enum UWBlocksetId {
    x0_CASTLE = 0x0,
    x1_DUNGEON = 0x1,
    x2_AGAHNIM = 0x2,
    x3_HOUSE = 0x3,
    x4_SANTUARY = 0x4,
    x5_EASTERN_PALACE = 0x5,
    x6_TOWER1 = 0x6,
    x7_TOWER_HERA = 0x7,
    x8_SWAMP_PALACE = 0x8,
    x9_SKULL_WOODS = 0x9,
    xA_THIEVES_TOWN = 0xA,
    xB_ICE_PALACE = 0xB,
    xC_MISERY_MIRE = 0xC,
    xD_TURTLE_ROCK = 0xD,
    xE_DESERT_PALACE = 0xE,
    xF_SAHASRAHLA = 0xF,
    x10_LINKS_HOUSE = 0x10,
    x11_SHOP = 0x11,
    x12_FAIRY_CAVE = 0x12,
    x13_GANON = 0x13,
    x14_CAVE = 0x14,
}
