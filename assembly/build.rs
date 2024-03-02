use compile::generate_asar_bindings;
use std::path::Path;

mod compile;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:warning=Detecting files");
    generate_asar_bindings(Path::new(&manifest_dir));
}
