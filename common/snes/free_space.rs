use log;

/// This describes a block of free space to put things in
pub struct FreeSpace {
    /// The bank the freespace is in.
    pub bank: u8,
    /// The position that can next be written.
    pub cursor: u16,
    /// The last position in the block of freespace.
    pub end: u16,
    /// Indicates that the freespace is completed exhausted.
    pub exhausted: bool,
}

impl FreeSpace {
    /// Creates a new block of freespace
    pub fn new(bank: u8, start: u16, end: u16) -> Self {
        FreeSpace {
            bank,
            end,
            cursor: start,
            exhausted: false,
        }
    }

    /// Returns the long address of the cursor.
    pub fn cursor_long(&self) -> usize {
        ((self.bank as usize) << 16) | self.cursor as usize
    }

    /// Returns the available range to be written.
    pub fn capacity(&self) -> u16 {
        self.end - self.cursor
    }

    /// Attempts to allocate the space.
    /// Returns None, if there isn't sufficient space.
    /// Returns Some(start, end) if there is.
    pub fn allocate(&mut self, length: u16) -> Option<usize> {
        log::debug!("Bank {:02X} -- allocates {:4} bytes", self.bank, length);
        let future_end_address = self.cursor as usize + length as usize;
        if future_end_address > self.end as usize || future_end_address > 0xFFFF {
            // Indicate this can't be allocated if it puts this past the safe range or it
            // exceeds the size of a bank.
            return None;
        }

        // Calculate the absolute addresses so they can be used to actually write.
        let alloc_start = ((self.bank as usize) << 16) | self.cursor as usize;
        // Update the cursor so it reports the new location.
        self.cursor = future_end_address as u16;
        self.exhausted = future_end_address == 0xFFFF || future_end_address >= self.end as usize;
        Some(alloc_start)
    }
}
