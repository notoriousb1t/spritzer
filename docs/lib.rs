// extern crate spritzer;
extern crate wasm_bindgen;
use std::panic;

use gloo_utils::format::JsValueSerdeExt;
use std::collections::BTreeMap;
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
    ow_balancing: i32,
    ow_enemy_shuffle: i32,
    shadow_bees: bool,
    uw_balancing: i32,
    uw_enemy_shuffle: i32,
    buffer: &[u8],
) -> Vec<u8> {
    let options = Z3Options {
        seed: seed.to_owned(),
        boss_shuffle,
        mushroom_shuffle,
        overworld_balancing: Balancing::from_repr(ow_balancing).unwrap(),
        overworld_enemy_shuffle: OverworldEnemyShuffle::from_repr(ow_enemy_shuffle).unwrap(),
        shadow_bees,
        underworld_balancing: Balancing::from_repr(uw_balancing).unwrap(),
        underworld_enemy_shuffle: UnderworldEnemyShuffle::from_repr(uw_enemy_shuffle).unwrap(),
    };

    randomize_zelda3(buffer, &options)
}

#[wasm_bindgen]
pub fn get_balancing_options() -> JsValue {
    let mut map: BTreeMap<i32, String> = BTreeMap::new();
    for option in Balancing::all() {
        map.insert(option as i32, option.to_string());
    }
    JsValue::from_serde(&map).unwrap()
}

#[wasm_bindgen]
pub fn get_ow_enemy_shuffle_options() -> JsValue {
    let mut map: BTreeMap<i32, String> = BTreeMap::new();
    for option in OverworldEnemyShuffle::all() {
        map.insert(option as i32, option.to_string());
    }
    JsValue::from_serde(&map).unwrap()
}

#[wasm_bindgen]
pub fn get_uw_enemy_shuffle_options() -> JsValue {
    let mut map: BTreeMap<i32, String> = BTreeMap::new();
    for option in UnderworldEnemyShuffle::all() {
        map.insert(option as i32, option.to_string());
    }
    JsValue::from_serde(&map).unwrap()
}
