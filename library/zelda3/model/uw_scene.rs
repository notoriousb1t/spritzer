use super::uw_door::UWDoorList;
use super::uw_layout::UWLayout;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct UWScene {
    /// Describes the general layout of a room.
    pub layout: UWLayout,
    /// Describes the entrances of a room.
    pub doors: UWDoorList,
}
