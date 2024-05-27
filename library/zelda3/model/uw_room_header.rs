use strum_macros::FromRepr;

use super::dungeon_blockset_id::UWBlocksetId;
use super::palette_id::PaletteId;
use super::room_logic::RoomLogic;
use super::spriteset_id::SpritesetId;
use super::uw_room_floor_id::UWFloorId;
use super::uw_room_id::UWRoomId;

#[derive(Clone)]
pub(crate) struct UnderworldRoomHeader {
    /// The type of background.
    pub bg2: RoomBackground,
    /// The collision type.
    pub collision: RoomCollision,
    /// True if the lights are out for this Underworld Room.
    pub light: bool,
    /// The palette to load for the Underworld Room.
    pub palette_id: PaletteId,
    /// Unused for now, probably the graphics id associated with the tileset.
    pub blockset_id: UWBlocksetId,
    /// The sprite graphics block associated. This constrains which Sprites can appear in this
    /// Underworld Room.
    pub spriteset_id: SpritesetId,
    /// The visual effect of the Underworld Room.
    pub effect: RoomEffect,
    /// The first tag. This provides data such as kill conditions.
    pub tag1: RoomLogic,
    /// The second tag. This provides data such as kill conditions.
    pub tag2: RoomLogic,
    /// The plane for hole/warp
    pub holewarp_plane: u8,
    /// The plane for stairs 1
    pub stairs1_plane: u8,
    /// The plane for stairs 2
    pub stairs2_plane: u8,
    /// The plane for stairs 3
    pub stairs3_plane: u8,
    /// The plane for stairs 4
    pub stairs4_plane: u8,
    /// The destination for warping
    pub holewarp: UWRoomId,
    /// The destination for stairs 0
    pub stairs1: UWRoomId,
    /// The destination for stairs 1
    pub stairs2: UWRoomId,
    /// The destination for stairs 2
    pub stairs3: UWRoomId,
    /// The destination for stairs 3
    pub stairs4: UWRoomId,
}

#[repr(u8)]
#[derive(Clone, Copy, FromRepr)]
pub(crate) enum RoomBackground {
    Off = 0, 
    Parallax = 1, 
    Dark = 2, 
    OnTop = 3, 
    Translucent = 4, 
    Addition = 5, 
    Normal = 6, 
    Transparent = 7, 
    DarkRoom = 8,
}

#[repr(u8)]
#[derive(Clone, Copy, FromRepr)]
pub(crate) enum RoomCollision {
    One = 0,
    Both = 1,
    BothWithScroll = 2,
    MovingFFloor = 3,
    MovingWater = 4,
}

#[repr(u8)]
#[derive(Clone, Copy, FromRepr)]
pub(crate) enum RoomEffect {
    Nothing = 0,
    One = 1,
    MovingFloor = 2,
    MovingWater = 3,
    Four = 4,
    RedFlashes = 5,
    TorchShowFloor = 6,
    GanonRoom = 7,
}


impl UnderworldRoomHeader {
    pub(crate) fn default() -> UnderworldRoomHeader {
        UnderworldRoomHeader {
            palette_id: PaletteId::x00_HYRULE_CASTLE,
            blockset_id: UWBlocksetId::x0_CASTLE,
            spriteset_id: SpritesetId::x40_Credits,
            effect: RoomEffect::Nothing,
            tag1: RoomLogic::x00_None,
            tag2: RoomLogic::x00_None,
            holewarp_plane: 0,
            stairs1_plane: 0,
            stairs2_plane: 0,
            stairs3_plane: 0,
            stairs4_plane: 0,
            holewarp: UWRoomId::x03_HOULIHAN_ROOM,
            stairs1: UWRoomId::x03_HOULIHAN_ROOM,
            stairs2: UWRoomId::x03_HOULIHAN_ROOM,
            stairs3: UWRoomId::x03_HOULIHAN_ROOM,
            stairs4: UWRoomId::x03_HOULIHAN_ROOM,
            bg2: RoomBackground::Off,
            collision: RoomCollision::One,
            light: false,
        }
    }
}

impl Into<u8> for UWFloorId {
    fn into(self) -> u8 {
        self as u8
    }
}
