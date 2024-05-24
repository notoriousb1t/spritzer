use strum_macros::FromRepr;

// Defines a region of an underworld room.
#[derive(PartialEq, Eq)]
pub(crate) enum Quadrant {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
}

impl Quadrant {
    pub(crate) fn from_point(x: u8, y: u8, size: u8) -> Option<Quadrant> {
        let midpoint = size / 2;
        if x < midpoint && y < midpoint {
            return Some(Quadrant::TopLeft);
        }
        if x >= midpoint && y < midpoint {
            return Some(Quadrant::TopRight);
        }
        if x < midpoint && y >= midpoint {
            return Some(Quadrant::BottomLeft);
        }
        if x >= midpoint && y >= midpoint {
            return Some(Quadrant::BottomRight);
        }
        None
    }
}

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

impl UWLayoutId {
    /// Returns the groups of quadrants based on the layout.
    pub(crate) fn quadrant_groups(&self) -> Vec<&[Quadrant]> {
        match self {
            UWLayoutId::X0_FourCorners => vec![
                &[Quadrant::TopLeft],
                &[Quadrant::TopRight],
                &[Quadrant::BottomLeft],
                &[Quadrant::BottomRight],
            ],
            UWLayoutId::X1_TwoColumns => vec![
                &[Quadrant::TopLeft, Quadrant::BottomLeft],
                &[Quadrant::TopRight, Quadrant::BottomRight],
            ],
            UWLayoutId::X2_LeftSplit => vec![
                &[Quadrant::TopLeft],
                &[Quadrant::TopRight, Quadrant::BottomRight],
                &[Quadrant::BottomLeft],
            ],
            UWLayoutId::X3_RightSplit => vec![
                &[Quadrant::TopLeft, Quadrant::BottomLeft],
                &[Quadrant::TopRight],
                &[Quadrant::BottomRight],
            ],
            UWLayoutId::X4_TwoRows => vec![
                &[Quadrant::TopLeft, Quadrant::TopRight],
                &[Quadrant::BottomLeft, Quadrant::BottomRight],
            ],
            UWLayoutId::X5_TopSplit => vec![
                &[Quadrant::TopLeft],
                &[Quadrant::TopRight],
                &[Quadrant::BottomLeft, Quadrant::BottomRight],
            ],
            UWLayoutId::X6_BottomSplit => vec![
                &[Quadrant::TopLeft, Quadrant::TopRight],
                &[Quadrant::BottomLeft],
                &[Quadrant::BottomRight],
            ],
            UWLayoutId::X7_Full => vec![&[
                Quadrant::TopLeft,
                Quadrant::TopRight,
                Quadrant::BottomLeft,
                Quadrant::BottomRight,
            ]],
        }
    }
}
