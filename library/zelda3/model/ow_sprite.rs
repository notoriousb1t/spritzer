use crate::zelda3::model::SpriteId;

#[derive(Clone, PartialEq, Hash, Eq)]
pub(crate) struct OWSprite {
    /// The Sprite placed in the Overworld Area.
    pub id: SpriteId,
    /// The x coordinate of the Sprite in the Overworld Area.
    pub x: u8,
    /// The y coordinate of the Sprite in the Overworld Area.
    pub y: u8,
}
