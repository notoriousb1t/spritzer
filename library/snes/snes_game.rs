use log::debug;

use super::free_space::FreeSpace;
use super::patch::Patch;
use super::snes_address::bytes_to_address;
use super::snes_address::int32_to_bytes;
use super::snes_address::snes_to_pc;

/// Manages reading and writing to the Game data.
pub(crate) struct SnesGame {
    pub(crate) buffer: Vec<u8>,
    pub(crate) free_space: Vec<FreeSpace>,
}

impl SnesGame {
    /// Constructs a new SNES game and expands it to the correct size.
    pub(crate) fn new(bytes: &[u8]) -> Self {
        Self {
            buffer: bytes.to_vec(),
            free_space: vec![],
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
            self.write_all(address, &vec![0x0; capacity]);
            debug!(
                "Clearing from ${:06X} to ${:06X}",
                address,
                address + capacity
            );
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
        let start = snes_to_pc(bank << 16);
        let end = snes_to_pc((bank << 16) | 0x10000);
        let range = &self.buffer[start..end];

        for (index, val) in range.iter().enumerate() {
            if index % 16 == 0 {
                debug!(
                    "PC[_{:06X}] @ #_{:06X}: ",
                    start + index,
                    (bank << 16) + index
                );
            }
            debug!(" {:02X}", val);
        }
    }

    /// Reads the byte from the SNES address.
    pub fn read(&self, address: usize) -> u8 {
        self.buffer[snes_to_pc(address)]
    }

    /// Reads a 16-bit integer from the address.
    pub fn read_int16(&self, address: usize) -> u16 {
        let data = self.read_all(address, 2);
        ((data[1] as u16) << 8) | data[0] as u16
    }

    /// Reads the byte from the SNES address.
    pub fn read_all(&self, address: usize, count: usize) -> &[u8] {
        let index = snes_to_pc(address);
        &self.buffer[index..(index + count)]
    }

    /// Reads a local pointer from the address and returns the address.
    pub fn read_pointer_int16(&self, address: usize) -> usize {
        let ptr = (self.read(address), self.read(address + 1));
        let address_bytes = int32_to_bytes(address);
        bytes_to_address([address_bytes[0], ptr.1, ptr.0])
    }

    /// Reads a global pointer from the address and returns the address.
    pub fn read_pointer_int24(&self, address: usize) -> usize {
        let ptr = (
            self.read(address),
            self.read(address + 1),
            self.read(address + 2),
        );
        bytes_to_address([ptr.2, ptr.1, ptr.0])
    }

    /// Writes a byte to the SNES address.
    pub fn write(&mut self, address: usize, value: u8) {
        self.buffer[snes_to_pc(address)] = value;
    }

    /// Writes all bytes using the SNES address.
    pub fn write_all(&mut self, address: usize, values: &[u8]) {
        for (i, val) in values.iter().enumerate() {
            self.buffer[snes_to_pc(address + i)] = *val;
        }
    }

    /// Writes a 16-bit integer to the address (little-endian).
    /// This is usually used for data. prefer write_local_pointer() for local pointers.
    pub fn write_int16(&mut self, address: usize, value: u16) {
        let bytes = int32_to_bytes(value as usize);
        self.write_all(address, &[bytes[2], bytes[1]]);
    }

    /// Writes a 24-bit integer to the address (little-endian).
    // The most common use case is global pointer. This is common for JSL and JML.
    pub fn write_int24(&mut self, address: usize, snes_location: usize) {
        let bytes = &int32_to_bytes(snes_location);
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
        debug!("Check bank {:?} for {} bytes", banks, values.len());
        if values.len() > 0xFFFF {
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
        let global_pointer = int32_to_bytes(target_address);
        let bank_location = int32_to_bytes(address);
        if bank_location[0] != global_pointer[0] {
            panic!(
                "Tried to write a local pointer in bank {} for bank {}",
                bank_location[0], global_pointer[0]
            );
        }

        // SNES is little-endian, so the least significant byte goes first. Local pointers assume
        // the current bank, so the most significant value is dropped.
        self.write_all(address, &[global_pointer[2], global_pointer[1]]);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::SnesGame;
    use crate::common::diff::Diff;

    impl SnesGame {
        /// Empty configuration for testing purposes.
        pub(crate) fn empty() -> Self {
            Self::new(&mut vec![0xFFu8; 0x400000])
        }

        /// Returns the deltas between the initial buffer and the current buffer.
        pub(crate) fn diff(&self, original: &[u8]) -> Vec<Diff<u8>> {
            Diff::compare(original, &self.buffer)
        }
    }

    #[test]
    fn diff_initial_state_returns_empty() {
        // This is a double check that diffing is setup correctly.
        let game = SnesGame::empty();
        let original = game.buffer.to_owned();
        assert_eq!(game.diff(&original), vec![]);
    }

    #[test]
    fn write_writes() {
        let mut game = SnesGame::empty();
        let original = game.buffer.to_owned();
        game.write(42, 0);

        let actual = game.diff(&original);
        let expectation = [Diff::value(42, 0xFF, 0)];
        assert_eq!(actual, expectation);
    }

    #[test]
    fn write_crc_writes() {
        let mut game = SnesGame::empty();
        let original = game.buffer.to_owned();
        game.write_crc();

        let actual = game.diff(&original);
        let expectation = [Diff::range(0x7FDC, vec![0xFF; 4], vec![253, 1, 2, 254])];
        assert_eq!(actual, expectation);
    }
}
