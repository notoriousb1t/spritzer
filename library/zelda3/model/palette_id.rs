use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::FromRepr;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Hash, FromRepr, EnumIter)]
#[allow(non_camel_case_types)]
pub(crate) enum PaletteId {
    x00_HYRULE_CASTLE = 0,
    x01_BLUE_CAVES = 1,
    x02_HOUSES = 2,
    x03_GREEN_DUNGEON = 3,
    x04_ICE_DUNGEON = 4,
    x05_DESERT_DUNGEON = 5,
    x06_TOWER_OF_HERA = 6,
    x07_CAVE = 7,
    x08_SWAMP_PALACE = 8,
    x09_DESERT_PALACE = 9,
    x0A_SWAMP_PALACE = 10,
    x0B_MISERY_MIRE = 11,
    x0C_AGAHNIMS_TOWER = 12,
    x0D_SKULL_WOODS = 13,
    x0E_CHURCH_SANCTUARY = 14,
    x0F_PALACE_OF_DARKNESS = 15,
    x10_PALACE_OF_DARKNESS_HELMASAUR = 16,
    x11_MISERY_MIRE = 17,
    x12_MISERY_MIRE_VITREUS = 18,
    x13_ICE_PALACE = 19,
    x14_ICE_PALACE_KHOLDSTARE = 20,
    x15_LINKS_HOUSE = 21,
    x16_FREESPACE = 22,
    x17_THIEVES_TOWN = 23,
    x18_TURTLE_ROCK = 24,
    x19_TURTLE_ROCK_TRINEXX = 25,
    x1A_GANONS_TOWER = 26,
    x1B_GANONS_TOWER = 27,
    x1C_SAHASRAHLAS_HOUSE = 28,
    x1D_SANCTUARY = 29,
    x1E_SHOP = 30,
    x1F_SHOP = 31,
    x20_CAVE = 32,
    x21_GANON = 33,
    x22_GREAT_FAIRY = 34,
    x23_BLIND = 35,
    x24_GANONS_TOWER = 36,
    x25_GANONS_TOWER = 37,
    x26_AGAHNIMS_TOWER = 38,
    x27_MIMICS_CAVE = 39,
    x28_GANONS_TOWER = 40,
}

impl Into<u8> for PaletteId {
    fn into(self) -> u8 {
        self as u8
    }
}