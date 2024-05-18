use super::sprite::Sprite;
use super::uw_room_id::UWRoomId;

#[derive(Clone, Debug)]
pub(crate) struct UWSpriteList {
    /// The Underworld Room this block of data describes. DO NOT MODIFY.
    pub room_id: UWRoomId,
    /// True if sprites are drawn in independent layers.
    pub sorted: bool,
    /// List of sprites in this Underworld Room.
    pub sprites: Vec<Sprite>,
}
