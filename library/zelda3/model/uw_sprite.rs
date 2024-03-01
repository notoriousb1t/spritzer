use super::sprite_id::SpriteId;

#[derive(Copy, Clone)]
pub(crate) struct UWSprite {
    /// The type of Sprite placed in the Underworld Room.."""
    pub id: SpriteId,
    /// The y coordinate of the Sprite in the Underworld Room. and some settings."""
    pub y_pos: u8,
    /// The x coordinate of the Sprite in the Underworld Room. and some settings."""
    pub x_pos: u8,
    /// True if this sprite is rendered on the bottom layer."""
    pub lower_layer: bool,
    /// Contains another Sprite. This should be a key or big key."""
    pub item: Option<SpriteId>,
    /// Subtype information about the sprite. Varies per sprite"""
    pub aux0: u8,
    /// A non-zero value indicates an overlord or key usually."""
    pub aux1: u8,
    /// May be set when performing shuffling/randomization to evaluate distance from the center of
    /// a group of enemies."""
    pub distance_from_midpoint: i32,
}

impl UWSprite {
    pub fn has_key(&self) -> bool {
        matches!(
            self.item,
            Some(SpriteId::xE4_SMALL_KEY) | Some(SpriteId::xE5_BIG_KEY)
        )
    }
}
