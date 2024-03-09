// extern crate spritzer;
extern crate wasm_bindgen;

pub mod options;

use std::panic;
use std::str::FromStr;

use log::info;
use options::DetectOptionsResult;
use options::Z3WasmOptions;
use spritzer::zelda3::detect_game;
use spritzer::zelda3::randomize_zelda3;
use spritzer::zelda3::Balancing;
use spritzer::zelda3::OverworldEnemyShuffle;
use spritzer::zelda3::UnderworldEnemyShuffle;
use spritzer::zelda3::Z3Options;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let logger = wasm_logger::Config::new(log::Level::Info);
    wasm_logger::init(logger);
}

#[wasm_bindgen]
pub fn process_zelda3(buffer: &[u8], options: Z3WasmOptions) -> Vec<u8> {
    info!("Processing Zelda3");

    let options = Z3Options {
        seed: options.seed.to_owned(),
        boss_shuffle: options.boss_shuffle,
        mushroom_shuffle: options.mushroom_shuffle,
        overworld_balancing: Balancing::from_str(&options.ow_balancing)
            .expect("Overworld Balancing should be valid"),
        overworld_enemy_shuffle: OverworldEnemyShuffle::from_str(&options.ow_enemy_shuffle)
            .expect("Overworld Enemy Shuffle should be valid"),
        shadow_bees: options.shadow_bees,
        underworld_balancing: Balancing::from_str(&options.uw_balancing)
            .expect("Underworld Balancing should be valid"),
        underworld_enemy_shuffle: UnderworldEnemyShuffle::from_str(&options.uw_enemy_shuffle)
            .expect("Underworld Enemy Shuffle should be valid"),
    };

    randomize_zelda3(buffer, &options)
}

#[wasm_bindgen]
pub fn detect_options(buffer: &[u8]) -> DetectOptionsResult {
    info!("Detecting Zelda3 Game");

    let game_info = detect_game(buffer);
    info!(
        "game_type = {}; supported = {}",
        &game_info.version, game_info.supported
    );
    let seed = Uuid::new_v4().as_hyphenated().to_string();

    let balancing_options = Balancing::all()
        .iter()
        .map(|it| it.to_string())
        .collect::<Vec<_>>();
    let ow_enemy_shuffle_options = OverworldEnemyShuffle::all()
        .iter()
        .map(|it| it.to_string())
        .collect::<Vec<_>>();
    let uw_enemy_shuffle_options = UnderworldEnemyShuffle::all()
        .iter()
        .map(|it| it.to_string())
        .collect::<Vec<_>>();

    DetectOptionsResult {
        supported: game_info.supported,
        game_type: game_info.version.to_string(),
        options: Z3WasmOptions {
            balancing_options,
            boss_shuffle: false,
            mushroom_shuffle: false,
            ow_balancing: Balancing::Balanced.to_string(),
            ow_enemy_shuffle_options,
            ow_enemy_shuffle: OverworldEnemyShuffle::Full.to_string(),
            seed: seed.to_string(),
            shadow_bees: false,
            uw_balancing: Balancing::Balanced.to_string(),
            uw_enemy_shuffle_options,
            uw_enemy_shuffle: UnderworldEnemyShuffle::Full.to_string(),
        },
    }
}
