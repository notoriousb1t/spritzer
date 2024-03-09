#![allow(dead_code)]
use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use spritzer::zelda3::{
    randomize_zelda3, Balancing, OverworldEnemyShuffle, UnderworldEnemyShuffle, Z3Options,
};
use std::fs;
use std::path::Path;

fn main() {
    setup_logging();

    let options = hard_mode();
    let current_dir = std::env::current_dir().unwrap();
    let input_path = current_dir.join("library/.testdata/p8.sfc");
    let input_bytes = read_file(input_path.as_ref());

    let output_bytes = randomize_zelda3(&input_bytes, &options);
    let output_path = current_dir.join("library/.build/output.sfc");

    write_file(output_path.as_path(), &output_bytes);
}

fn setup_logging() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .expect("Logger failed to setup");
}

fn read_file(file_path: &Path) -> Vec<u8> {
    info!("Reading '{:?}'", file_path.as_os_str());
    match fs::read(file_path) {
        Ok(file_contents) => file_contents,
        Err(err) => {
            error!("Error reading file: {}", err);
            panic!("Could not read file!");
        }
    }
}

fn write_file(file_path: &Path, data: &[u8]) {
    info!("Writing '{:?}'", file_path.as_os_str());
    match fs::write(file_path, &data) {
        Ok(()) => {
            info!("Wrote file successfully");
        }
        Err(err) => {
            error!("Error writing to file: {}", err);
        }
    }
}

/// Useful when testing basic patches / world flattening.
fn base_options() -> Z3Options {
    Z3Options {
        boss_shuffle: false,
        mushroom_shuffle: false,
        overworld_balancing: Balancing::Random,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Vanilla,
        underworld_balancing: Balancing::Random,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Vanilla,
        seed: "test".to_owned(),
        shadow_bees: false,
    }
}

/// Useful for testing easier runs.
fn easy_mode() -> Z3Options {
    Z3Options {
        boss_shuffle: false,
        mushroom_shuffle: false,
        overworld_balancing: Balancing::Casual,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Simple,
        underworld_balancing: Balancing::Casual,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Simple,
        seed: "test".to_owned(),
        shadow_bees: false,
    }
}

/// Useful for testing Inverted overworld mode.
fn inverted_mode() -> Z3Options {
    Z3Options {
        boss_shuffle: true,
        mushroom_shuffle: true,
        overworld_balancing: Balancing::Balanced,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Inverted,
        underworld_balancing: Balancing::Balanced,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Chaos,
        seed: "test".to_owned(),
        shadow_bees: false,
    }
}

/// Useful for testing the preferred experience.
fn balanced_mode() -> Z3Options {
    Z3Options {
        boss_shuffle: true,
        mushroom_shuffle: false,
        overworld_balancing: Balancing::Balanced,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Full,
        underworld_balancing: Balancing::Balanced,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Full,
        seed: "test".to_owned(),
        shadow_bees: false,
    }
}

/// Stop trying so hard!
fn hard_mode() -> Z3Options {
    Z3Options {
        boss_shuffle: true,
        mushroom_shuffle: true,
        overworld_balancing: Balancing::Hero,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Insanity,
        underworld_balancing: Balancing::Hero,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Insanity,
        seed: "test".to_owned(),
        shadow_bees: true,
    }
}
