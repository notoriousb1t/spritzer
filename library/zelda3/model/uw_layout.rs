use super::uw_layout_id::UWLayoutId;
use super::uw_object::UWObject;

/// Represents the object and layout data of an Underworld Room.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct UWLayout {
    pub floor1: u8,
    pub floor2: u8,
    pub layout: UWLayoutId,
    // Preserving the last two bytes of layout until I figure out
    // what it is for.
    pub aux0: u8,
    pub layers: Vec<Vec<UWObject>>,
}
