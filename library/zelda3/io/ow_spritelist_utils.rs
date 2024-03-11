use assembly::zelda3::Symbol;

use crate::zelda3::model::OWRoomId;
use crate::zelda3::model::OWStateId;

pub(super) fn get_sprite_graphics_address(id: OWRoomId, overworld_id: OWStateId) -> usize {
    if id == OWRoomId::x40_MASTER_SWORD_UNDER_BRIDGE {
        if overworld_id == OWStateId::LIGHT_WORLD_V2 {
            return Symbol::OwSpecialGraphics as usize + 1;
        }
        return Symbol::OwSpecialGraphics as usize;
    }
    if id == OWRoomId::x41_ZORAS_DOMAIN {
        if overworld_id == OWStateId::LIGHT_WORLD_V2 {
            return Symbol::OwSpecialGraphics as usize + 3;
        }
        return Symbol::OwSpecialGraphics as usize + 2;
    }

    let version_offset: usize = match overworld_id {
        OWStateId::LIGHT_WORLD_V0 => 0,
        OWStateId::LIGHT_WORLD_V1 => 1,
        OWStateId::LIGHT_WORLD_V2 => 2,
        OWStateId::DARK_WORLD_V1 => 3,
        OWStateId::DARK_WORLD_V2 => 3,
    };
    Symbol::UwGraphics as usize + (version_offset * 0x40) + id as usize
}

pub(super) fn get_palette_address(id: OWRoomId, overworld_id: OWStateId) -> usize {
    if id == OWRoomId::x40_MASTER_SWORD_UNDER_BRIDGE {
        return Symbol::OwSpecialPalette as usize;
    }
    if id == OWRoomId::x41_ZORAS_DOMAIN {
        return Symbol::OwSpecialPalette as usize + 1;
    }
    let version_offset: usize = match overworld_id {
        OWStateId::LIGHT_WORLD_V0 => 0,
        OWStateId::LIGHT_WORLD_V1 => 1,
        OWStateId::LIGHT_WORLD_V2 => 2,
        OWStateId::DARK_WORLD_V1 => 3,
        OWStateId::DARK_WORLD_V2 => 3,
    };
    Symbol::UwGraphics as usize + 0x100 + (version_offset * 0x40) + id as usize
}

pub(super) fn get_sprite_pointer(id: OWRoomId, overworld_id: OWStateId) -> usize {
    let base_address = match overworld_id {
        OWStateId::LIGHT_WORLD_V0 => Symbol::OwSpritePtrs as usize,
        OWStateId::LIGHT_WORLD_V1 => Symbol::OwSpritePtrs as usize + 0x80,
        OWStateId::DARK_WORLD_V1 => Symbol::OwSpritePtrs as usize + 0x100,
        OWStateId::LIGHT_WORLD_V2 => Symbol::OwSpritePtrs as usize + 0x1A0,
        OWStateId::DARK_WORLD_V2 => Symbol::OwSpritePtrs as usize + 0x220,
    };

    base_address
        + match id {
            OWRoomId::x40_MASTER_SWORD_UNDER_BRIDGE => 0x100,
            OWRoomId::x41_ZORAS_DOMAIN => 0x100 + 2,
            _ => id as usize * 2,
        }
}
