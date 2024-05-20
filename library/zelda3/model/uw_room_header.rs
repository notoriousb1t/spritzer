use super::dungeon_blockset_id::UWBlocksetId;
use super::palette_id::PaletteId;
use super::room_logic::RoomLogic;
use super::spriteset_id::SpritesetId;
use super::uw_room_floor_id::UWFloorId;
use super::uw_room_id::UWRoomId;

#[derive(Clone)]
pub(crate) struct UnderworldRoomHeader {
    /// The Underworld Room this block of data describes. DO NOT MODIFY.
    pub id: UWRoomId,
    /// True if the lights are out for this Underworld Room.
    pub bg2_property: u8,
    /// The palette to load for the Underworld Room.
    pub palette_id: PaletteId,
    /// Unused for now, probably the graphics id associated with the tileset.
    pub blockset_id: UWBlocksetId,
    /// The sprite graphics block associated. This constrains which Sprites can appear in this
    /// Underworld Room.
    pub spriteset_id: SpritesetId,
    /// The visual effect of the Underworld Room.
    pub bgmove: u8,
    /// The first tag. This provides data such as kill conditions.
    pub tag1: RoomLogic,
    /// The second tag. This provides data such as kill conditions.
    pub tag2: RoomLogic,
    /// The pattern/type of the top floor. (Water, Tiled, etc.)
    pub planes1: UWFloorId,
    /// The pattern/type of the bottom floor. (Water, Tiled, etc.)
    pub planes2: UWFloorId,
    /// The destination for warping
    pub warp: UWRoomId,
    /// The destination for stairs 0
    pub stairs0: UWRoomId,
    /// The destination for stairs 1
    pub stairs1: UWRoomId,
    /// The destination for stairs 2
    pub stairs2: UWRoomId,
    /// The destination for stairs 3
    pub stairs3: UWRoomId,
}

impl Into<u8> for UWFloorId {
    fn into(self) -> u8 {
        self as u8
    }
}
