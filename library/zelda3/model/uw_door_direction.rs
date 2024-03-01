use strum_macros::FromRepr;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, FromRepr, Hash)]
pub(crate) enum UWDoorDirection {
    X0_North = 0,
    X1_South = 1,
    X2_West = 2,
    X3_East = 3,
}
