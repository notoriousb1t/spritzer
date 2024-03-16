use super::spritesheet_id::SpritesheetId;
use super::spriteset_id::SpritesetId;

/// Contains up to 4 SpriteSheets that load in individual sprites.
/// There can only be one loaded at a time, so this must have all the
/// correct SpriteSheets for any Sprite that has non-persistent graphics in memory.
#[derive(Clone)]
pub(crate) struct Spriteset {
    pub id: SpritesetId,
    pub sheets: [SpritesheetId; 4],
}
