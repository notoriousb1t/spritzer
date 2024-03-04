// extern crate spritzer;
extern crate wasm_bindgen;

pub mod options;

use std::panic;
use std::str::FromStr;

use log::info;
use options::{DetectOptionsResult, Z3WasmOptions};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use spritzer::snes::SnesGame;
use spritzer::zelda3::{
    randomize_zelda3, Balancing, OverworldEnemyShuffle, UnderworldEnemyShuffle, Z3Options,
};

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
        overworld_balancing: Balancing::from_str(&options.ow_balancing).unwrap(),
        overworld_enemy_shuffle: OverworldEnemyShuffle::from_str(&options.ow_enemy_shuffle)
            .unwrap(),
        shadow_bees: options.shadow_bees,
        underworld_balancing: Balancing::from_str(&options.uw_balancing).unwrap(),
        underworld_enemy_shuffle: UnderworldEnemyShuffle::from_str(&options.uw_enemy_shuffle)
            .unwrap(),
    };

    randomize_zelda3(buffer, &options)
}

#[wasm_bindgen]
pub fn detect_options(buffer: &[u8]) -> DetectOptionsResult {
    info!("Detecting Zelda3 Game");

    let game = SnesGame::new(buffer);
    let title = game.get_game_title();

    let (game_type, supported) = detect_game(title);
    info!("game_type = {}; title = {}; supported = {}", game_type, title, supported);
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
        supported,
        game_type: game_type.to_owned(),
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

fn detect_game(title: &str) -> (&str, bool) {
    if title.starts_with("ZELDANODENSETSU") {
        ("Zelda JP 1.0", false)
    } else if title.starts_with("THE LEGEND OF ZELDA") {
        ("Zelda US", false)
    } else if title.starts_with("AP") {
        ("Archipelago", true)
    } else if title.starts_with("VT") {
        ("A Link to the Past Randomizer", false)
    } else {
        ("Unknown Game", false)
    }
}
