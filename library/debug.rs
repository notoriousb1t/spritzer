#![allow(dead_code)]
use std::fs;
use std::fs::File;
use std::path::Path;

use log::error;
use log::info;
use log::LevelFilter;
use simplelog::ColorChoice;
use simplelog::CombinedLogger;
use simplelog::ConfigBuilder;
use simplelog::TermLogger;
use simplelog::TerminalMode;
use simplelog::WriteLogger;
use spritzer::zelda3::randomize_zelda3;
use spritzer::zelda3::Balancing;
use spritzer::zelda3::OverworldEnemyShuffle;
use spritzer::zelda3::UnderworldEnemyShuffle;
use spritzer::zelda3::Z3Options;

fn main() {
    setup_logging();

    let options = chaos_mode("kholdstare".to_owned());
    let current_dir = std::env::current_dir().expect(&format!("Could not get current directory"));
    let input_path = current_dir.join("./library/testdata/p8.sfc");
    let input_bytes = read_file(input_path.as_ref());

    let output_bytes = randomize_zelda3(&input_bytes, &options);
    let output_path = current_dir.join("./library/build/output.sfc");

    write_file(output_path.as_path(), &output_bytes);
}

fn setup_logging() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            ConfigBuilder::new()
                .set_thread_level(LevelFilter::Off)
                .set_time_level(LevelFilter::Off)
                .set_location_level(LevelFilter::Off)
                .build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            ConfigBuilder::new()
                .set_thread_level(LevelFilter::Off)
                .set_time_level(LevelFilter::Off)
                .set_location_level(LevelFilter::Off)
                .build(),
            File::create("./library/build/output.sfc.log").unwrap(),
        ),
    ])
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
fn base_options(seed: String) -> Z3Options {
    Z3Options {
        boss_shuffle: false,
        pot_shuffle: true,
        mushroom_shuffle: false,
        killable_thieves: false,
        overworld_balancing: Balancing::Random,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Vanilla,
        underworld_balancing: Balancing::Random,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Vanilla,
        seed,
        shadow_bees: false,
    }
}

/// Useful for testing easier runs.
fn easy_mode(seed: String) -> Z3Options {
    Z3Options {
        boss_shuffle: false,
        pot_shuffle: true,
        mushroom_shuffle: false,
        killable_thieves: true,
        overworld_balancing: Balancing::Casual,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Chaos,
        underworld_balancing: Balancing::Casual,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Chaos,
        seed,
        shadow_bees: false,
    }
}

/// Useful for testing Inverted overworld mode.
fn inverted_mode(seed: String) -> Z3Options {
    Z3Options {
        boss_shuffle: true,
        mushroom_shuffle: true,
        pot_shuffle: true,
        killable_thieves: true,
        overworld_balancing: Balancing::Balanced,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Inverted,
        underworld_balancing: Balancing::Balanced,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Chaos,
        seed,
        shadow_bees: false,
    }
}

/// Useful for testing the preferred experience.
fn chaos_mode(seed: String) -> Z3Options {
    Z3Options {
        overworld_balancing: Balancing::Random,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Chaos,
        underworld_balancing: Balancing::Random,
        boss_shuffle: false,
        pot_shuffle: true,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Chaos,
        killable_thieves: true,
        mushroom_shuffle: true,
        shadow_bees: true,
        seed,
    }
}

/// Useful for testing the preferred experience.
fn balanced_mode(seed: String) -> Z3Options {
    Z3Options {
        boss_shuffle: true,
        mushroom_shuffle: false,
        killable_thieves: true,
        pot_shuffle: true,
        overworld_balancing: Balancing::Balanced,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Full,
        underworld_balancing: Balancing::Balanced,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Full,
        seed,
        shadow_bees: false,
    }
}

/// Stop trying so hard!
fn hard_mode(seed: String) -> Z3Options {
    Z3Options {
        boss_shuffle: true,
        mushroom_shuffle: true,
        killable_thieves: true,
        pot_shuffle: true,
        overworld_balancing: Balancing::Hero,
        overworld_enemy_shuffle: OverworldEnemyShuffle::Insanity,
        underworld_balancing: Balancing::Hero,
        underworld_enemy_shuffle: UnderworldEnemyShuffle::Insanity,
        seed,
        shadow_bees: true,
    }
}
