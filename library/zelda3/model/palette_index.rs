use strum_macros::Display;
use strum_macros::FromRepr;

#[repr(u8)]
#[derive(FromRepr, Copy, Clone, Debug, Display, PartialEq, Eq)]
pub(crate) enum PaletteIndex {
    X0None = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3Red = 0x3,
    X4Blue = 0x4,
    X5Green = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
    XA = 0xA,
    XB = 0xB,
    XC = 0xC,
    XD = 0xD,
    XENoir = 0xE,
    XF = 0xF,
}
