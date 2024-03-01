use strum_macros::FromRepr;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, FromRepr, Hash)]
/// Defines the base layout of the room. There are 8 built-in layouts. It
/// is possible to define additonal custom layouts via hooks.
pub(crate) enum UWLayoutId {
    /// Split into equal regions.
    X0_FourCorners = 0b000,
    /// Split into two equal columns.
    X1_TwoColumns = 0b001,
    /// The left side has two rooms, the right is a full column.
    X2_LeftSplit = 0b010,
    /// The right side has two rooms, the right is a full column.
    X3_RightSplit = 0b011,
    /// Split into two equal rows.
    X4_TwoRows = 0b100,
    /// The top is two equal rooms. The bottom is a full row.
    X5_TopSplit = 0b101,
    /// The bottom is two equal rooms. The top is a full row.
    X6_BottomSplit = 0b110,
    /// it is a large full room (no divisions.)
    X7_Full = 0b111,
}
