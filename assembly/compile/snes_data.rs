use std::cmp::max;
use std::cmp::min;

const BLANK: u8 = 0xFF;

/// Creates fake SNES data.
pub(super) fn create_empty_snes_data() -> Vec<u8> {
    let mut buffer = vec![BLANK; 4 * 1024 * 1024];

    // 0x7FC0 = 21 length Cartridge title. (ignored)

    // Rom Speed and Memory Map Mode
    buffer[0x7FD5] = 0x20;
    // Chipset
    buffer[0x7FD6] = 1;
    // ROM size in killobytes 1 << N
    buffer[0x7FD7] = 12;
    // RAM size in killobytes 1 << N
    buffer[0x7FD8] = 5;
    // Country (NTSC/PAL)
    buffer[0x7FD9] = 1;
    // Developer ID
    buffer[0x7FDA] = 0;
    // ROM Version.
    buffer[0x7FDB] = 0;
    // Checksum
    buffer[0x7FDC] = 0;
    buffer[0x7FDC] = 0;
    // Compliment (Checksum ^ 0xFFFF)
    buffer[0x7FDE] = 0;
    buffer[0x7FDF] = 0;
    buffer
}

/// Returns the deltas between the original ROM and the data provided.
pub(super) fn get_snes_deltas(data: &Vec<u8>) -> Vec<(usize, Vec<u8>)> {
    let mut deltas: Vec<(usize, Vec<u8>)> = vec![];
    let mut last_address: Option<usize> = None;
    let mut current_slice: Option<Vec<u8>> = None;

    let size = data.len();
    for (index, byte) in data.iter().enumerate() {
        if index <= 0x7FDF {
            // Skip the header.
            continue;
        }

        if let Some(address) = last_address {
            // Treat 8 consequetive blank characters as a terminator.
            let upper = max(index, min(index + 2, size));
            let is_boundary = (index..upper).all(|i| data[i] == BLANK);

            if is_boundary {
                let slice = current_slice.to_owned();
                current_slice = None;
                last_address = None;
                deltas.push((address, slice.unwrap()));
                continue;
            }

            if let Some(slice) = &mut current_slice {
                slice.push(*byte);
            }
            continue;
        }
        if byte != &BLANK {
            last_address = Some(index);
            current_slice = Some(vec![*byte]);
            continue;
        }
    }

    if let Some(address) = last_address {
        if let Some(slice) = current_slice {
            deltas.push((address, slice.to_vec()));
        }
    }
    deltas
}

pub(super) fn pc_address_to_snes_address(pc_address: usize) -> usize {
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
