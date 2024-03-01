pub fn bytes_to_address(byte_values: [u8; 3]) -> usize {
    ((byte_values[0] as usize) << 16) | ((byte_values[1] as usize) << 8) | byte_values[2] as usize
}

pub fn snes_to_pc(address: usize) -> usize {
    (address & 0x7FFF) + ((address / 2) & 0xFF8000)
}

pub fn int32_to_bytes(address: usize) -> [u8; 3] {
    [
        ((address >> 16) & 0xFF) as u8,
        ((address >> 8) & 0xFF) as u8,
        (address & 0xFF) as u8,
    ]
}
