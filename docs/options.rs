use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DetectOptionsResult {
    pub supported: bool,
    pub(crate) game_type: String,
    pub(crate) options: Z3WasmOptions,
}

#[wasm_bindgen]
impl DetectOptionsResult {
    #[wasm_bindgen(getter)]
    pub fn game_type(&self) -> String {
        self.game_type.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn options(&self) -> Z3WasmOptions {
        self.options.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Z3WasmOptions {
    pub boss_shuffle: bool,
    pub killable_thieves: bool,
    pub mushroom_shuffle: bool,
    pub shadow_bees: bool,
    pub(crate) seed: String,
    pub(crate) ow_balancing: String,
    pub(crate) ow_enemy_shuffle: String,
    pub(crate) uw_balancing: String,
    pub(crate) uw_enemy_shuffle: String,
    pub(crate) balancing_options: Vec<String>,
    pub(crate) ow_enemy_shuffle_options: Vec<String>,
    pub(crate) uw_enemy_shuffle_options: Vec<String>,
}

#[wasm_bindgen]
impl Z3WasmOptions {
    #[wasm_bindgen(getter)]
    pub fn seed(&self) -> String {
        self.seed.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_seed(&mut self, seed: String) {
        self.seed = seed;
    }

    #[wasm_bindgen(getter)]
    pub fn ow_balancing(&self) -> String {
        self.ow_balancing.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn ow_enemy_shuffle(&self) -> String {
        self.ow_enemy_shuffle.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_ow_enemy_shuffle(&mut self, ow_enemy_shuffle: String) {
        self.ow_enemy_shuffle = ow_enemy_shuffle;
    }

    #[wasm_bindgen(setter)]
    pub fn set_ow_balancing(&mut self, ow_balancing: String) {
        self.ow_balancing = ow_balancing;
    }

    #[wasm_bindgen(getter)]
    pub fn uw_enemy_shuffle(&self) -> String {
        self.uw_enemy_shuffle.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_uw_enemy_shuffle(&mut self, uw_enemy_shuffle: String) {
        self.uw_enemy_shuffle = uw_enemy_shuffle;
    }

    #[wasm_bindgen(getter)]
    pub fn uw_balancing(&self) -> String {
        self.uw_balancing.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_uw_balancing(&mut self, uw_balancing: String) {
        self.uw_balancing = uw_balancing;
    }

    #[wasm_bindgen(getter)]
    pub fn balancing_options(&self) -> Vec<String> {
        self.balancing_options.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn ow_enemy_shuffle_options(&self) -> Vec<String> {
        self.ow_enemy_shuffle_options.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn uw_enemy_shuffle_options(&self) -> Vec<String> {
        self.uw_enemy_shuffle_options.clone()
    }
}
