use super::uw_door_direction::UWDoorDirection;
use super::uw_door_position::UWDoorPosition;
use super::uw_door_style::UWDoorStyle;

pub(crate) type UWDoorList = Vec<UWDoor>;

/// The configuration for a single door in a room.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct UWDoor {
    /// The visual style of the door (regular, waterfall, etc.)
    pub style: UWDoorStyle,
    /// The direction the player moves while entering the door.
    pub direction: UWDoorDirection,
    /// The placement of the door. There are 12 slots.
    pub position: UWDoorPosition,
}

impl UWDoor {
    pub(crate) fn default() -> UWDoor {
        UWDoor {
            style: UWDoorStyle::X00_NormalDoor,
            direction: UWDoorDirection::X0_North,
            position: UWDoorPosition::X00_Position00,
        }
    }
}