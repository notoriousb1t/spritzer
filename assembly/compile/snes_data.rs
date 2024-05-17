use std::cmp::max;
use std::cmp::min;

const BLANK: u8 = 0xFF;

/// Returns the deltas between the original ROM and the data provided.
pub fn get_snes_deltas(data: &Vec<u8>) -> Vec<(usize, Vec<u8>)> {
    let mut deltas: Vec<(usize, Vec<u8>)> = vec![];
    let mut last_address: Option<usize> = None;
    let mut current_slice: Option<Vec<u8>> = None;

    let size = data.len();
    for (index, byte) in data.iter().enumerate() {
        if index >= 0x7FD5 && index < 0x7FE0 {
            // Skip the header.
            continue;
        }

        if let Some(address) = last_address {
            // Treat 2 consequetive blank characters as a terminator.
            // The clear flaw with this is that 0xFF 0xFF could be within data.
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
