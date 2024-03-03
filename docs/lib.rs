// extern crate spritzer;
extern crate wasm_bindgen;
use std::panic;
use std::str::FromStr;

use gloo_utils::format::JsValueSerdeExt;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use spritzer::zelda3::{
    randomize_zelda3, Balancing, OverworldEnemyShuffle, UnderworldEnemyShuffle, Z3Options,
};

#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn process_zelda3(
    seed: &str,
    boss_shuffle: bool,
    mushroom_shuffle: bool,
    ow_balancing: &str,
    ow_enemy_shuffle: &str,
    shadow_bees: bool,
    uw_balancing: &str,
    uw_enemy_shuffle: &str,
    buffer: &[u8],
) -> Vec<u8> {
    let options = Z3Options {
        seed: seed.to_owned(),
        boss_shuffle,
        mushroom_shuffle,
        overworld_balancing: Balancing::from_str(ow_balancing).unwrap(),
        overworld_enemy_shuffle: OverworldEnemyShuffle::from_str(ow_enemy_shuffle).unwrap(),
        shadow_bees,
        underworld_balancing: Balancing::from_str(uw_balancing).unwrap(),
        underworld_enemy_shuffle: UnderworldEnemyShuffle::from_str(uw_enemy_shuffle).unwrap(),
    };

    randomize_zelda3(buffer, &options)
}

#[wasm_bindgen]
pub fn get_options() -> JsValue {
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

    let map: HashMap<String, Vec<String>> = HashMap::from_iter([
        ("balancingOptions".to_string(), balancing_options),
        (
            "owEnemyShuffleOptions".to_string(),
            ow_enemy_shuffle_options,
        ),
        (
            "uwEnemyShuffleOptions".to_string(),
            uw_enemy_shuffle_options,
        ),
    ]);
    JsValue::from_serde(&map).unwrap()
}
