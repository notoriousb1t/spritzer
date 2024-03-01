use strum_macros::FromRepr;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, FromRepr, Hash)]
/// Describes the placement of doors in a room. There are 12 slots total. They are are in
/// rows (or columns!) of three. There are two rows at the top of the direction and two rows in
/// the middle of the direction.
pub(crate) enum UWDoorPosition {
    X00_Position00 = 0x00,
    X02_Position01 = 0x02,
    X04_Position02 = 0x04,
    X06_Position10 = 0x06,
    X08_Position11 = 0x08,
    X0A_Position12 = 0x0A,
    X0C_Position20 = 0x0C,
    X0E_Position21 = 0x0E,
    X10_Position22 = 0x10,
    X12_Position30 = 0x12,
    X14_Position31 = 0x14,
    X16_Position32 = 0x16,
}
