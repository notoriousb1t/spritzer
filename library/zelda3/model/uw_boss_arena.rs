use super::uw_room_id::UWRoomId;

pub(crate) struct Offsets {
    // X offet for the arena (for arenas not in the top left corner)
    pub x: u8,
    // Y offset for the arena (for arenas not in the top left corner)
    pub y: u8,
    // True if the boss is typically on a lower layer (Moldorm's arena in Hera).
    pub lower_layer: bool,
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) enum ArenaPosition {
    Center = 0,
    TopLeft = 1,
    TopRight = 2,
    BottomLeft = 3,
    BottomRight = 4,
}

/// Describes the x and y offsets from the top left corner of the super tile to the
/// individual room in the layout.
/// An arena is assumed to be 9x9 for the purposes of layout (although it can certainly be bigger)
/// TODO: remove this once Underworld Rooms have a notion of layout.
pub(crate) fn get_arena_offsets(source_id: UWRoomId, target_id: UWRoomId) -> Offsets {
    match get_arena_position(source_id, target_id) {
        ArenaPosition::Center => center_arena(),
        ArenaPosition::TopLeft => top_left_arena(),
        ArenaPosition::TopRight => top_right_arena(),
        ArenaPosition::BottomLeft => bottom_left_arena(),
        ArenaPosition::BottomRight => bottom_right_arena(),
    }
}

/// Returns the general position of the boss in its respective arena.
pub(crate) fn get_arena_position(source_id: UWRoomId, target_id: UWRoomId) -> ArenaPosition {
    match target_id {
        UWRoomId::xC8_EASTERN_PALACE_ARMOS_KNIGHTS_BOSS => ArenaPosition::BottomRight,
        UWRoomId::x33_DESERT_PALACE_LANMOLAS_BOSS => ArenaPosition::BottomLeft,
        UWRoomId::x07_TOWER_OF_HERA_MOLDORM_BOSS => match is_center_problematic(source_id) {
            true => ArenaPosition::TopRight,
            false => ArenaPosition::Center,
        },
        UWRoomId::x20_AGAHNIM_S_TOWER_AGAHNIM_BOSS => ArenaPosition::BottomLeft,
        UWRoomId::x5A_PALACE_OF_DARKNESS_HELMASAUR_KING_BOSS => ArenaPosition::BottomRight,
        UWRoomId::x06_SWAMP_PALACE_ARRGHUS_BOSS => ArenaPosition::BottomLeft,
        UWRoomId::x29_SKULL_WOODS_MOTHULA_BOSS => ArenaPosition::BottomRight,
        UWRoomId::xAC_THIEVES_TOWN_BLIND_THE_THIEF_BOSS => ArenaPosition::BottomRight,
        UWRoomId::xDE_ICE_PALACE_KHOLDSTARE_BOSS => ArenaPosition::TopRight,
        UWRoomId::x90_MISERY_MIRE_VITREOUS_BOSS => ArenaPosition::BottomLeft,
        UWRoomId::xA4_TURTLE_ROCK_TRINEXX_BOSS => ArenaPosition::BottomLeft,
        UWRoomId::x0D_GANON_S_TOWER_AGAHNIM2_BOSS => ArenaPosition::BottomLeft,
        UWRoomId::x1C_GANON_S_TOWER_ICE_ARMOS => match is_center_problematic(source_id) {
            true => ArenaPosition::TopRight,
            false => ArenaPosition::Center,
        },
        UWRoomId::x6C_GANON_S_TOWER_LANMOLAS_ROOM => ArenaPosition::BottomLeft,
        UWRoomId::x4D_GANON_S_TOWER_MOLDORM_ROOM => ArenaPosition::Center,
        _ => {
            panic!("Unsupported arena {}", target_id);
        }
    }
}

fn is_center_problematic(source_id: UWRoomId) -> bool {
    matches!(
        source_id,
        UWRoomId::x06_SWAMP_PALACE_ARRGHUS_BOSS
            | UWRoomId::x1C_GANON_S_TOWER_ICE_ARMOS
            | UWRoomId::x29_SKULL_WOODS_MOTHULA_BOSS
            | UWRoomId::x33_DESERT_PALACE_LANMOLAS_BOSS
            | UWRoomId::x5A_PALACE_OF_DARKNESS_HELMASAUR_KING_BOSS
            | UWRoomId::x6C_GANON_S_TOWER_LANMOLAS_ROOM
            | UWRoomId::xA4_TURTLE_ROCK_TRINEXX_BOSS
            | UWRoomId::xAC_THIEVES_TOWN_BLIND_THE_THIEF_BOSS
            | UWRoomId::xC8_EASTERN_PALACE_ARMOS_KNIGHTS_BOSS
            | UWRoomId::xDE_ICE_PALACE_KHOLDSTARE_BOSS
    )
}

fn top_right_arena() -> Offsets {
    Offsets {
        x: 16,
        y: 0,
        lower_layer: false,
    }
}

fn top_left_arena() -> Offsets {
    Offsets {
        x: 0,
        y: 0,
        lower_layer: false,
    }
}

fn bottom_left_arena() -> Offsets {
    Offsets {
        x: 0,
        y: 16,
        lower_layer: false,
    }
}

fn bottom_right_arena() -> Offsets {
    Offsets {
        x: 16,
        y: 16,
        lower_layer: false,
    }
}

fn center_arena() -> Offsets {
    Offsets {
        x: 8,
        y: 8,
        lower_layer: false,
    }
}
