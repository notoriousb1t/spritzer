#!allow(clippy::single_char_pattern)]

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use common::pc_address_to_snes_address;
use common::RomMode;
use common::RomSize;
use common::SnesGame;

use super::fileutils::cleanup_temp_file;
use super::fileutils::find_files;
use super::fileutils::get_hash;
use super::fileutils::read_all_bytes;
use super::fileutils::read_lines;
use super::fileutils::replace_segment;
use super::snes_data::get_snes_deltas;

const SETTING_PREFIX: &str = "setting_";

pub fn generate_asar_bindings(project_path: &Path) {
    println!("cargo:warning=Crawling {:?}", project_path);

    if let Ok(files) = find_files(project_path, "mod.asm") {
        for file_path in files {
            let asm_path: &Path = file_path.as_ref();
            println!("cargo:warning=ASM Detected: {:?}", asm_path);
            rebuild_if_necessary(project_path, asm_path);
        }
    }
}

fn get_rust_path(_project_path: &Path, asm_path: &Path) -> PathBuf {
    replace_segment(asm_path, "src", "generated")
        .parent()
        .expect("Path to have a parent")
        .with_extension("rs")
}

fn rebuild_if_necessary(project_path: &Path, asm_path: &Path) {
    let rs_path = get_rust_path(project_path, asm_path);
    let hash = get_hash(asm_path);
    if rs_path.exists() && !get_hash_from_rust_file(&rs_path).is_empty() {
        return;
    }

    process_mod_asm(project_path, asm_path, hash);
}

/// Process each mod.asm file and produce a mod.rs file.
fn process_mod_asm(project_path: &Path, asm_path: &Path, hash: String) {
    println!("cargo:warning=Processing ASM {:?}", asm_path);

    let asar_cmd = get_asar_path(project_path);
    println!("cargo:warning=Asar command: {:?}", asar_cmd);

    let bin_path = asm_path.with_extension("bin");
    let sym_path = asm_path.with_extension("sym");
    let rs_path = get_rust_path(project_path, asm_path);

    cleanup_temp_file(&bin_path.clone());

    // Create a new file
    println!("cargo:warning=Creating dummy ROM {:?}", &bin_path);
    let mut file = File::create(&bin_path).expect("Could not create dummy file");
    let game = SnesGame::new(RomMode::FastLoRom, RomSize::Size4mb);
    file.write_all(&game.buffer)
        .expect("File could not be written");

    let output_result = std::process::Command::new(asar_cmd)
        .current_dir(project_path)
        .args([
            "--fix-checksum=off",
            "--no-title-check",
            "--symbols=nocash",
            "-v",
        ])
        .arg(asm_path)
        .arg(&bin_path)
        .output();

    if let Err(err) = output_result {
        println!("cargo:warning=Cleaning up {:?}", bin_path);
        cleanup_temp_file(&bin_path);
        panic!("{}", format!("{:#?}", err));
    }

    let output = output_result.unwrap();
    // Check the output of the command
    if !output.status.success() {
        println!("cargo:warning=Cleaning up {:?}", bin_path);
        cleanup_temp_file(&bin_path);
        panic!("{}", format!("{:#?}", output));
    }
    println!("cargo:warning=Compiled ASM successfully!");
    let symbols = extract_symbols(&sym_path);
    let deltas = get_snes_deltas(&read_all_bytes(&bin_path));

    let file_contents = generate_rust_file(hash, &symbols, &deltas);
    let mut file = File::create(rs_path).unwrap();
    file.write_all(file_contents.as_bytes()).unwrap();

    println!("cargo:warning=Cleaning up {:?}", bin_path);
    cleanup_temp_file(&sym_path);
    cleanup_temp_file(&bin_path);
}

fn generate_rust_file(
    hash: String,
    symbols: &Vec<(String, usize)>,
    deltas: &[(usize, Vec<u8>)],
) -> String {
    // Write out symbols to rust file.
    let mut output = String::new();
    // Add hash to top of file.
    output.push_str(format!("//! {}\n", hash).as_str());
    output.push_str("//! Generated from asm file. Remove top line to regenerate. \n");
    // Suppress warnings that don't matter for this file.
    output.push_str("#![allow(dead_code, non_camel_case_types)]");
    // Add imports
    output.push_str("use strum_macros::{Display, EnumIter, FromRepr};\n\n");
    // Declare as usize and derive common enum properties.
    output.push_str("#[repr(usize)]\n");
    output.push_str(
        "#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, EnumIter, FromRepr)]\n",
    );
    // Define all the values as Name = SNES_ADDRESS_USIZE.
    output.push_str("pub enum SettingAddress {\n");
    for (symbol_name, location) in symbols {
        if symbol_name.starts_with(SETTING_PREFIX) {
            output.push_str(
                format!(
                    "    {} = {:#02X},\n",
                    symbol_name.strip_prefix(SETTING_PREFIX).unwrap(),
                    location
                )
                .as_str(),
            );
        }
    }
    output.push_str("}\n\n");

    // Add in an implicit operator for usize since these are used primarily as addresses.
    output.push_str("impl From<SettingAddress> for usize {\n");
    output.push_str("    fn from(value: SettingAddress) -> usize {\n");
    output.push_str("        value as usize\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    // Write out patch deltas with each SNES location as the key.
    output.push_str("pub fn get_patch_data() -> Vec<(usize, Vec<u8>)> {\n");
    output.push_str("  vec![\n");

    for (address, data) in deltas.iter() {
        let snes_address = pc_address_to_snes_address(*address);
        output.push_str(format!("    (0x{:02X}, vec![\n", snes_address).as_str());
        for chunk in data.chunks(8) {
            output.push_str("        ");
            for it in chunk {
                output.push_str(format!("0x{:02X}, ", it).as_str());
            }
            output.push('\n');
        }
        output.push_str("    ]),\n");
    }

    output.push_str("  ]\n");
    output.push_str("}\n");

    output
}

/// Returns a vector of tuples indicating symbol names and their respective locations.
fn extract_symbols(path: &Path) -> Vec<(String, usize)> {
    let mut symbols: Vec<(String, usize)> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            if line.is_empty() || line.starts_with(';') {
                continue;
            }
            let (addr_string, symbol_name) = line.split_once(' ').unwrap();
            let pc_address = usize::from_str_radix(addr_string, 16).unwrap();
            symbols.push((symbol_name.to_string(), pc_address));
        }
    }
    symbols
}

/// The binary should be present in the assembly folder.
/// Either asar.exe for windows or asar for linux.
fn get_asar_path(project_path: &Path) -> PathBuf {
    project_path.join("asar")
}

fn get_hash_from_rust_file(path: &Path) -> String {
    if let Ok(mut lines) = read_lines(path) {
        let maybe_line = lines.next();
        if maybe_line.is_none() {
            return "".to_string();
        }
        if let Some((_str1, str2)) = maybe_line.unwrap().unwrap().split_once(' ') {
            return str2.to_string();
        }
        return "".to_string();
    }
    "".to_string()
}
