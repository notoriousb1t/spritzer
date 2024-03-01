use std::path::Path;
use compile::generate_asar_bindings;

mod compile;

fn main() {
    println!("cargo:warning=Detecting files");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    generate_asar_bindings(Path::new(&manifest_dir));
}
