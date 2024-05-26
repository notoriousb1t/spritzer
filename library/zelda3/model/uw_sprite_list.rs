use super::sprite::Sprite;

#[derive(Clone, Debug)]
pub(crate) struct UWSpriteList {
    /// True if sprites are drawn in independent layers.
    pub sorted: bool,
    /// List of sprites in this Underworld Room.
    pub sprites: Vec<Sprite>,
}
