use std::cmp::max;
use std::cmp::min;

use super::rom_type::RomType;

fn snes_to_pc(address: usize) -> usize {
    (address & 0x7FFF) + ((address / 2) & 0xFF8000)
}

pub fn int24_to_bytes(address: usize) -> [u8; 3] {
    [
        ((address >> 16) & 0xFF) as u8,
        ((address >> 8) & 0xFF) as u8,
        (address & 0xFF) as u8,
    ]
}

pub fn bytes_to_int24(byte_values: [u8; 3]) -> usize {
    ((byte_values[0] as usize) << 16) | ((byte_values[1] as usize) << 8) | byte_values[2] as usize
}

/// Resolves the address for the ROM mode of the SNES game.
pub fn snes_to_physical(mode: RomType, address: usize) -> usize {
    match mode {
        RomType::FastLoRom => snes_to_pc(address % 0x80_0000),
        _ => address,
    }
}

pub fn pc_address_to_snes_address(pc_address: usize) -> usize {
    // The u32eger in big endian bytes.
    let mut bytes = [
        (pc_address >> 16) & 0xFF,
        (pc_address >> 8) & 0xFF,
        pc_address & 0xFF,
    ];
    // Convert to SNES bytes.
    bytes[0] = max(0, min(bytes[0] * 2, 0xFF));
    if bytes[1] >= 0x80 {
        bytes[0] += 1;
    } else {
        bytes[1] += 0x80;
    }
    (bytes[0] << 16) | (bytes[1] << 8) | bytes[2]
}
