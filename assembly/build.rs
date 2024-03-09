use std::path::Path;

use compile::generate_asar_bindings;

mod compile;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR is a required environment variable");
    println!("cargo:warning=Detecting files");
    generate_asar_bindings(Path::new(&manifest_dir));
}
