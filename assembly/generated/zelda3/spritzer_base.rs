//! a5238479a9f3c6f14e8bd799407f0233e593d0c4a041ddd7a402e1b1767e76f4
//! Generated from asm file. Remove top line to regenerate.
#![allow(dead_code, non_camel_case_types)]
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::FromRepr;

#[repr(usize)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, EnumIter, FromRepr)]
pub enum Symbol {
    AncillaAdd_FallingPrize_hera_room_id = 0x98C11,
    DamageClass = 0x6F42D,
    DamageSubclass = 0xDB8F1,
    DoorPtrs = 0x1F83C0,
    Entrances = 0x2C577,
    Killable = 0x698070,
    LayoutPtrs = 0x1F8000,
    Moldorm_EyeCount = 0x1DDBB3,
    OWRoomEmpty = 0x9CB41,
    OwSpecialGraphics = 0x2E575,
    OwSpecialPalette = 0x2E596,
    OwSpritePtrs = 0x9C881,
    RoomData_SpritePointers_Ref0 = 0x9C298,
    RoomSpritesEnd = 0x9EC9C,
    RoomSpritesStart = 0x9CB42,
    SpriteSettings = 0xDB080,
    Spriteset = 0xDB97,
    UWHeaderBank = 0x1B5E7,
    UWHeaderRef0 = 0x1B5DD,
    UWRoomEmpty = 0x9EC9D,
    UwGraphics = 0xFA41,
    UwHeaderPtrs = 0x4F1E2,
    UwSpritePtrs = 0x9D62E,
}

impl From<Symbol> for usize {
    fn from(value: Symbol) -> usize {
        value as usize
    }
}

pub fn get_patch_data() -> Vec<(usize, Vec<u8>)> {
    vec![
        (0x687CB, vec![0xB6, 0x91]),
        (0x688CA, vec![0x22, 0x60, 0x80, 0x69]),
        (0x691B6, vec![0x22, 0x88, 0x80, 0x69]),
        (0x6EC08, vec![0x22, 0xB2, 0x80, 0x69, 0xEA]),
        (0x6EDA6, vec![0x22, 0x70, 0x80, 0x69]),
        (0x9C9E3, vec![0x41, 0xCB]),
        (0x9CB03, vec![0x41, 0xCB]),
        (
            0x9DB19,
            vec![0x14, 0x07, 0x84, 0x1C, 0x03, 0x83, 0x1C, 0x0C, 0x84],
        ),
        (0x9DEEB, vec![0x04, 0x07, 0x84]),
        (0x9DEF4, vec![0x08, 0x04, 0x83, 0x08, 0x0B, 0x84]),
        (0x9E199, vec![0x06, 0x0A, 0x84, 0x09, 0x06, 0x84]),
        (0x9E1A5, vec![0x18, 0x03, 0x84]),
        (0x9E1AE, vec![0x1B, 0x0C, 0xB8, 0x15, 0x17, 0xB8]),
        (0x9E1BA, vec![0x1B, 0x18, 0xB8]),
        (
            0x9EBCD,
            vec![
                0x14, 0x07, 0xB8, 0x14, 0x08, 0xB8, 0x14, 0x0C, 0xB8, 0x1A, 0x0C, 0xB8,
            ],
        ),
        (0xDB9A9, vec![0x00]),
        (
            0x1DD88E,
            vec![
                0x22, 0xC7, 0x80, 0x69, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA,
            ],
        ),
        (0x1DDBB3, vec![0x07]),
        (0x1E8BB1, vec![0x9D, 0xC7]),
        (0x1EC7C5, vec![0xB8]),
        (0x1EC801, vec![0xB8]),
        (
            0x698060,
            vec![
                0xAD, 0x8E, 0x04, 0xC9, 0xAC, 0xD0, 0x04, 0x5C, 0x81, 0xA0, 0x1D, 0x22, 0x90, 0xA0,
                0x1D, 0x6B, 0xBD, 0x20, 0x0E, 0xC9, 0xFF, 0xF0, 0x07, 0xC9, 0xB8, 0xF0, 0x03, 0x4C,
                0x83, 0x80, 0xA9, 0x83, 0x4C, 0x83, 0x80, 0xC2, 0x20, 0x0A, 0x0A, 0x6B, 0xBD, 0x20,
                0x0E, 0xC9, 0xB8, 0xF0, 0x01, 0x6B, 0xA9, 0x83, 0x9D, 0x20, 0x0E, 0x22, 0x18, 0xB8,
                0x0D, 0xA9, 0xB8, 0x9D, 0x20, 0x0E, 0xBD, 0xAA, 0x0C, 0x29, 0xFB, 0x09, 0x80, 0x9E,
                0xAA, 0x0C, 0xFE, 0xA0, 0x0D, 0xBD, 0x20, 0x0E, 0x9E, 0xAA, 0x0C, 0x6B, 0xBD, 0x20,
                0x0E, 0xC9, 0xB8, 0xD0, 0x05, 0xA9, 0x83, 0x4C, 0xBE, 0x80, 0x9D, 0x20, 0x0E, 0xBD,
                0x20, 0x0E, 0xC9, 0x7A, 0x6B, 0xDA, 0xA6, 0xB3, 0xAA, 0xA5, 0x90, 0x18, 0x69, 0x04,
                0x00, 0x85, 0x90, 0xA5, 0x92, 0x18, 0x69, 0x01, 0x00, 0x85, 0x92, 0xCA, 0x10, 0xED,
                0xFA, 0x6B,
            ],
        ),
    ]
}
