use std::str::from_utf8;

use log;
use strum_macros::Display;
use strum_macros::FromRepr;

use super::free_space::FreeSpace;
use super::patch::Patch;
use super::rom_type::RomType;
use super::snes_address::bytes_to_int24;
use super::snes_address::int24_to_bytes;
use super::snes_address::snes_to_physical;
use crate::Diff;

const TITLE_ADDRESS: usize = 0xFFC0;
const TYPE_ADDRESS: usize = 0xFFD5;
const SIZE_ADDRESS: usize = 0xFFD7;

/// Manages reading and writing to the Game data.
pub struct SnesGame {
    pub mode: RomType,
    pub buffer: Vec<u8>,
    pub free_space: Vec<FreeSpace>,
}

#[repr(u8)]
#[derive(Clone, Copy, FromRepr, Display)]
#[allow(dead_code)]
pub enum RomSize {
    Size1mb = 0xA,
    Size2mb = 0xB,
    Size4mb = 0xC,
    Size8mb = 0xD,
    Size16mb = 0xE,
    Size32mb = 0xF,
}

impl SnesGame {
    /// Empty configuration for testing purposes.
    pub fn new(rom_type: RomType, rom_size: RomSize) -> Self {
        let mut data = vec![0xFFu8; (1 << rom_size as usize) * 1024];
        data[0x7FD5] = rom_type as u8;
        data[0x7FD7] = rom_size as u8;
        Self::from_bytes(&mut data)
    }

    /// Constructs a new SNES game and expands it to the correct size.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mode_value = bytes[snes_to_physical(RomType::FastLoRom, TYPE_ADDRESS)];
        let mode = RomType::from_repr(mode_value)
            .expect(&format!("SnesMode {:02X} is invalid", mode_value));

        log::info!("Address mode={}", mode);

        Self {
            mode,
            buffer: bytes.to_vec(),
            free_space: vec![],
        }
    }

    /// Returns the deltas between the initial buffer and the current buffer.
    pub fn diff(&self, original: &[u8]) -> Vec<Diff<u8>> {
        Diff::compare(original, &self.buffer)
    }

    pub fn get_size(&self) -> RomSize {
        let value = self.read(SIZE_ADDRESS);
        return RomSize::from_repr(value).expect(&format!("SnesSize {:02X} is invalid", value));
    }

    pub fn set_game_title(&mut self, title: &str) {
        let values = title.as_bytes().into_iter().cloned().collect::<Vec<_>>();

        self.write_all(TITLE_ADDRESS, &values);
    }

    pub fn get_game_title(&self) -> &str {
        from_utf8(self.read_all(TITLE_ADDRESS, 21)).expect("SNES game should have title")
    }

    /// Resizes the ROM and updates the header.. Empty space is filled with 0xFF.
    pub fn resize(&mut self, size: RomSize) {
        let new_size = (1 << (size as usize)) * 1024;
        if new_size <= self.buffer.len() {
            log::info!("Resizing skipped {} <= {}", new_size, self.buffer.len());
        } else {
            log::info!("Resizing game {} >= {}", new_size, self.buffer.len());
            self.buffer.resize(new_size, 0x0);
            self.write(SIZE_ADDRESS, size as u8);
        }
    }

    /// This fills all known freespace with 0s and merges declared freespace.
    pub fn deallocate(&mut self) {
        // Sort free space to the larger banks are used first. The general wisdom is that SNES games
        // should write data in the upper banks and code in the lower banks. (Not sure if
        // this is still true when there is no physical hardware, but this is easy enough either
        // way)
        self.free_space
            .sort_by_key(|free| -(((free.bank as isize) << 16) | free.cursor as isize));

        // Precalculate ranges before looping write. (C)
        let areas_to_clear = self
            .free_space
            .iter()
            .map(|free| {
                let address = free.cursor_long();
                let capacity = free.capacity() as usize;
                (address, capacity)
            })
            .collect::<Vec<_>>();

        for (address, capacity) in areas_to_clear {
            self.write_all(address, &vec![0; capacity]);
            log::debug!("Clearing  ${:06X}..${:06X}", address, address + capacity);
        }
    }

    /// Finds the next bank with the required capacity.
    pub fn find_capacity(&self, capacity: usize) -> Option<u8> {
        self.free_space
            .iter()
            .find(|free| free.capacity() as usize >= capacity)
            .map(|free| free.bank)
    }

    /// Signals that the area provided can be used to write other data.
    pub fn mark(&mut self, bank: u8, start: u16, end: u16) {
        // NOTE: this does not handle joining blocks of freespace. That sophistication
        // isn't needed at this point.
        self.free_space.push(FreeSpace::new(bank, start, end));
    }

    /// Applies the patches provided.
    pub fn patch(&mut self, patches: &[Patch]) {
        for patch in patches {
            self.write_all(patch.address, &patch.values);
        }
    }

    /// Allows printing out the contents of the buffer for a specific SNES bank. This is largely
    /// useful for debugging.
    #[allow(dead_code)]
    pub fn print_bank(&self, bank: usize) {
        let start = snes_to_physical(self.mode, bank << 16);
        let end = snes_to_physical(self.mode, (bank << 16) | 0x10000);
        let range = &self.buffer[start..end];

        for (index, val) in range.iter().enumerate() {
            if index % 16 == 0 {
                log::debug!(
                    "PC[_{:06X}] @ #_{:06X}: ",
                    start + index,
                    (bank << 16) + index
                );
            }
            log::debug!(" {:02X}", val);
        }
    }

    /// Reads the byte from the SNES address.
    pub fn read(&self, address: usize) -> u8 {
        self.buffer[snes_to_physical(self.mode, address)]
    }

    /// Reads a 16-bit integer from the address.
    pub fn read_int16(&self, address: usize) -> u16 {
        let data = self.read_all(address, 2);
        ((data[1] as u16) << 8) | data[0] as u16
    }

    /// Reads the byte from the SNES address.
    pub fn read_all(&self, address: usize, count: usize) -> &[u8] {
        let index = snes_to_physical(self.mode, address % 0x80_0000);
        &self.buffer[index..(index + count)]
    }

    /// Reads a local pointer from the address and returns the address.
    pub fn read_pointer_int16(&self, address: usize) -> usize {
        let ptr = (self.read(address), self.read(address + 1));
        let address_bytes = int24_to_bytes(address);
        bytes_to_int24([address_bytes[0] % 0x80, ptr.1, ptr.0])
    }

    /// Reads a global pointer from the address and returns the address.
    pub fn read_pointer_int24(&self, address: usize) -> usize {
        let ptr = (
            self.read(address),
            self.read(address + 1),
            self.read(address + 2),
        );
        bytes_to_int24([ptr.2 % 0x80, ptr.1, ptr.0])
    }

    /// Writes a byte to the SNES address.
    pub fn write(&mut self, address: usize, value: u8) {
        self.buffer[snes_to_physical(self.mode, address)] = value;
    }

    /// Writes all bytes using the SNES address.
    pub fn write_all(&mut self, address: usize, values: &[u8]) {
        for (i, val) in values.iter().enumerate() {
            self.buffer[snes_to_physical(self.mode, address + i)] = *val;
        }
    }

    /// Writes a 16-bit integer to the address (little-endian).
    /// This is usually used for data. prefer write_local_pointer() for local pointers.
    pub fn write_int16(&mut self, address: usize, value: u16) {
        let bytes = int24_to_bytes(value as usize);
        self.write_all(address, &[bytes[2], bytes[1]]);
    }

    /// Writes a 24-bit integer to the address (little-endian).
    /// The most common use case is global pointer. This is common for JSL and JML.
    /// This writes at a bank offset of 0x80 when FastLoRom is detected.
    pub fn write_pointer(&mut self, address: usize, snes_location: usize) {
        let offset = match self.mode {
            RomType::FastLoRom => 0x80_0000,
            _ => 0,
        };
        let bytes = &int24_to_bytes(snes_location + offset);
        self.write_all(address, &[bytes[2], bytes[1], bytes[0]]);
    }

    /// Writes the CRC for the SNES game.
    pub fn write_crc(&mut self) {
        let mut crc: u32 = 0x01FE;
        for b in self.buffer[..0x7FDC].iter() {
            crc += *b as u32;
        }
        for b in self.buffer[0x7FE0..].iter() {
            crc += *b as u32;
        }
        crc &= 0xFFFF;

        let inv = crc ^ 0xFFFF;
        self.buffer[0x7FDC] = (inv & 0xFF) as u8;
        self.buffer[0x7FDD] = ((inv >> 8) & 0xFF) as u8;
        self.buffer[0x7FDE] = (crc & 0xFF) as u8;
        self.buffer[0x7FDF] = ((crc >> 8) & 0xFF) as u8;
    }

    /// Attempts to write to freespace in the banks provided. This should be used to write data that
    /// can be accessed via long addresses such as JSL and JML.
    /// Returns None if there was insufficient space
    /// Returns Some(long_address) if successful.
    pub fn write_data(&mut self, banks: &[u8], values: &[u8]) -> Option<usize> {
        if values.len() > 0xFFFF {
            log::debug!("Check bank {:?} for {} bytes", banks, values.len());
            panic!("The values cannot fit into a single bank!");
        }

        let mut start_position = None;
        for space in self.free_space.iter_mut() {
            if !banks.contains(&space.bank) {
                continue;
            }
            if space.exhausted || values.len() > space.capacity() as usize {
                continue;
            }
            let maybe_position = space.allocate(values.len() as u16);
            if maybe_position.is_none() {
                continue;
            }
            start_position = maybe_position;
            break;
        }

        if let Some(start) = start_position {
            self.write_all(start, values);
        }

        start_position
    }

    /// Writes a local pointer using the SNES address.
    /// This will panic if the caller attempts to write a local pointer to a bank where
    /// it is not located.
    pub fn write_pointer_int16(&mut self, address: usize, target_address: usize) {
        let global_pointer = int24_to_bytes(target_address);
        // SNES is little-endian, so the least significant byte goes first. Local pointers assume
        // the current bank, so the most significant value is dropped.
        self.write_all(address, &[global_pointer[2], global_pointer[1]]);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::SnesGame;
    use crate::snes::rom_type::RomType;
    use crate::Diff;
    use crate::RomSize;

    #[test]
    fn diff_initial_state_returns_empty() {
        // This is a double check that diffing is setup correctly.
        let game = SnesGame::new(RomType::FastLoRom, RomSize::Size4mb);
        let original = game.buffer.to_owned();
        assert_eq!(game.diff(&original), vec![]);
    }

    #[test]
    fn write_writes() {
        let mut game = SnesGame::new(RomType::FastLoRom, RomSize::Size4mb);
        let original = game.buffer.to_owned();
        game.write(42, 0);

        let actual = game.diff(&original);
        let expectation = [Diff::value(42, 0xFF, 0)];
        assert_eq!(actual, expectation);
    }

    #[test]
    fn write_crc_writes() {
        let mut game = SnesGame::new(RomType::FastLoRom, RomSize::Size4mb);
        let original = game.buffer.to_owned();
        game.write_crc();

        let actual = game.diff(&original);
        let expectation = [Diff::range(0x7FDC, vec![0xFF; 4], vec![191, 3, 64, 252])];
        assert_eq!(actual, expectation);
    }
}
