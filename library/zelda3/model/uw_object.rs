use super::uw_object_id::UWObjectId;

/// Represents an instance of a object in the underworld.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct UWObject {
    /// The unique id representing the type of object.
    pub id: UWObjectId,
    /// The x coordinate of the object.
    pub x: u8,
    /// The y coordinate of the object.
    pub y: u8,
    /// The size profile of the object.
    pub width: u8,
    /// The size profile of the object.
    pub height: u8,
}

impl UWObject {
    pub(crate) fn from_value(object_value: u16, x: u8, y: u8, w: u8, h: u8) -> Self {
        UWObject {
            x,
            y,
            width: w,
            height: h,
            id: UWObjectId::from_repr(object_value).unwrap(),
        }
    }

    pub(crate) fn from_id(object_id: UWObjectId, x: u8, y: u8, width: u8, height: u8) -> Self {
        UWObject {
            x,
            y,
            width,
            height,
            id: object_id,
        }
    }
}
