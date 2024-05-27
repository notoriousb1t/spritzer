use super::uw_layout_id::UWLayoutId;
use super::uw_object::UWObject;
use super::UWFloorId;

/// Represents the object and layout data of an Underworld Room.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct UWLayout {
    pub floor1: UWFloorId,
    pub floor2: UWFloorId,
    pub layout: UWLayoutId,
    // Preserving the last two bytes of layout until I figure out
    // what it is for.
    pub aux0: u8,
    pub layers: Vec<Vec<UWObject>>,
}

impl UWLayout {
    pub(crate) fn default() -> UWLayout {
        UWLayout {
            floor1: UWFloorId::x00_GROUND_NEUTRAL,
            floor2: UWFloorId::x00_GROUND_NEUTRAL,
            layout: UWLayoutId::X0_FourCorners,
            aux0: 0,
            layers: vec![],
        }
    }
}