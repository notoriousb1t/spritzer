use super::sprite_id::SpriteId;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Sprite {
    /// The type of Sprite placed in the Room.
    pub id: SpriteId,
    /// The y coordinate of the Sprite in the Room.
    pub y: u8,
    /// The x coordinate of the Sprite in the Room.
    pub x: u8,
    /// True if this sprite is rendered on the bottom layer in the underworld.
    pub is_lower_layer: bool,
    /// Contains another Sprite in the underworld. This should be a key or big key.
    pub item: Option<SpriteId>,
    /// Subtype information about the sprite in the underworld. Varies per sprite.
    pub aux0: Option<u8>,
    /// A non-zero value indicates an overlord or key usually in the underworld.
    pub aux1: Option<u8>,
}

impl Sprite {
    pub fn has_key(&self) -> bool {
        matches!(
            self.item,
            Some(SpriteId::xE4_SMALL_KEY) | Some(SpriteId::xE5_BIG_KEY)
        )
    }
}
