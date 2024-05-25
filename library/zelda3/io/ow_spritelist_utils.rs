use crate::zelda3::model::OWRoomId;
use crate::zelda3::model::OWStateId;
use crate::zelda3::Addresses;

pub(super) fn get_sprite_graphics_address(addresses: &Addresses, id: OWRoomId, overworld_id: OWStateId) -> usize {
    if id == OWRoomId::x40_MASTER_SWORD_UNDER_BRIDGE {
        if overworld_id == OWStateId::LIGHT_WORLD_V2 {
            return addresses.owspecial_graphics + 1;
        }
        return addresses.owspecial_graphics;
    }
    if id == OWRoomId::x41_ZORAS_DOMAIN {
        if overworld_id == OWStateId::LIGHT_WORLD_V2 {
            return addresses.owspecial_graphics + 3;
        }
        return addresses.owspecial_graphics + 2;
    }

    let version_offset: usize = match overworld_id {
        OWStateId::LIGHT_WORLD_V0 => 0,
        OWStateId::LIGHT_WORLD_V1 => 1,
        OWStateId::LIGHT_WORLD_V2 => 2,
        OWStateId::DARK_WORLD_V1 => 3,
        OWStateId::DARK_WORLD_V2 => 3,
    };
    addresses.uwgraphics + (version_offset * 0x40) + id as usize
}

pub(super) fn get_palette_address(addresses: &Addresses, id: OWRoomId, overworld_id: OWStateId) -> usize {
    if id == OWRoomId::x40_MASTER_SWORD_UNDER_BRIDGE {
        return addresses.owspecial_palette;
    }
    if id == OWRoomId::x41_ZORAS_DOMAIN {
        return addresses.owspecial_palette + 1;
    }
    let version_offset: usize = match overworld_id {
        OWStateId::LIGHT_WORLD_V0 => 0,
        OWStateId::LIGHT_WORLD_V1 => 1,
        OWStateId::LIGHT_WORLD_V2 => 2,
        OWStateId::DARK_WORLD_V1 => 3,
        OWStateId::DARK_WORLD_V2 => 3,
    };
    addresses.uwgraphics + 0x100 + (version_offset * 0x40) + id as usize
}

pub(super) fn get_sprite_pointer(addresses: &Addresses, id: OWRoomId, overworld_id: OWStateId) -> usize {
    let base_address = match overworld_id {
        OWStateId::LIGHT_WORLD_V0 => addresses.owsprite_ptrs,
        OWStateId::LIGHT_WORLD_V1 => addresses.owsprite_ptrs + 0x80,
        OWStateId::DARK_WORLD_V1 => addresses.owsprite_ptrs + 0x100,
        OWStateId::LIGHT_WORLD_V2 => addresses.owsprite_ptrs + 0x1A0,
        OWStateId::DARK_WORLD_V2 => addresses.owsprite_ptrs + 0x220,
    };

    base_address
        + match id {
            OWRoomId::x40_MASTER_SWORD_UNDER_BRIDGE => 0x100,
            OWRoomId::x41_ZORAS_DOMAIN => 0x100 + 2,
            _ => id as usize * 2,
        }
}
