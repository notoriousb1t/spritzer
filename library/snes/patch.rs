use super::snes_address::int32_to_bytes;

#[derive(Debug, Clone)]
pub struct Patch {
    /// The SNES address to write this data.
    pub address: usize,
    /// The values to write.
    pub values: Vec<u8>,
}

impl Patch {
    pub fn of(address: usize, value: u8) -> Self {
        Self::from(address, &[value])
    }

    pub fn from(address: usize, values: &[u8]) -> Self {
        Patch {
            address,
            values: values.to_vec(),
        }
    }

    /// Adds a patch that changes a local pointer.
    pub fn update_local_pointer(address: usize, pointer: usize) -> Self {
        let bytes = int32_to_bytes(pointer);
        Patch {
            address,
            values: vec![bytes[2], bytes[1]],
        }
    }
}
