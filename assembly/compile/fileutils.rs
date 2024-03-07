use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Read;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

/// A quick utility to finding files with particular names.
pub(super) fn find_files(dir_path: &Path, filename: &str) -> std::io::Result<Vec<PathBuf>> {
    let entries = std::fs::read_dir(dir_path)?;
    let mut result = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_results = find_files(&path, filename)?;
            result.extend(sub_results);
        } else if path.is_file() && path.file_name().unwrap().to_str().unwrap() == filename {
            result.push(path.clone());
        }
    }

    Ok(result)
}

/// Cleans up a file if it exists.
pub(super) fn cleanup_temp_file(path: &Path) {
    // Cleanup old temp file.
    if path.exists() {
        if let Err(err) = std::fs::remove_file(path) {
            panic!("cargo:error=Error cleaning up file. {}", err);
        }
    }
}

/// Reads the lines of a file and returns an iterator.
pub(super) fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub(super) fn read_all_bytes(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec![];
    let _ = file.read_to_end(&mut buffer);
    buffer
}

pub(super) fn get_hash(path: &Path) -> String {
    sha256::try_digest(std::path::Path::new(path)).unwrap()
}

pub(super) fn replace_segment(path: &Path, segment_to_replace: &str, new_segment: &str) -> PathBuf {
    // Find the index of the segment to replace
    let segments: Vec<_> = path.components().map(|c| c.as_os_str()).collect();
    let idx = segments
        .iter()
        .position(|&s| s == segment_to_replace)
        .expect("Path segment to exist");

    // Construct the new path with the replaced segment
    let mut new_path = PathBuf::new();
    for (i, segment) in segments.iter().enumerate() {
        if i == idx {
            new_path.push(new_segment);
        } else {
            new_path.push(segment);
        }
    }
    new_path
}
