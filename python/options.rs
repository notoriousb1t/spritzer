use std::str::FromStr;

use pyo3::prelude::*;
use spritzer::zelda3::Balancing;
use spritzer::zelda3::OverworldEnemyShuffle;
use spritzer::zelda3::UnderworldEnemyShuffle;
use spritzer::zelda3::Z3Options;

/// The options exposed to consuming python scripts and libraries.
/// Note: please use primitives instead of enums, etc.
#[pyclass(name = "Options")]
pub struct OptionsPy {
    pub seed: String,
    #[pyo3(get, set)]
    pub boss_shuffle: bool,
    #[pyo3(get, set)]
    pub killable_thieves: bool,
    #[pyo3(get, set)]
    pub mushroom_shuffle: bool,
    #[pyo3(get, set)]
    pub overworld_balancing: String,
    #[pyo3(get, set)]
    pub overworld_enemy_shuffle: String,
    #[pyo3(get, set)]
    pub pot_shuffle: bool,
    #[pyo3(get, set)]
    pub shadow_bees: bool,
    #[pyo3(get, set)]
    pub underworld_balancing: String,
    #[pyo3(get, set)]
    pub underworld_enemy_shuffle: String,
}

#[pymethods]
impl OptionsPy {
    #[new]
    pub fn new(seed: String) -> Self {
        OptionsPy {
            seed,
            boss_shuffle: false,
            killable_thieves: false,
            mushroom_shuffle: false,
            overworld_balancing: Balancing::Random.to_string(),
            overworld_enemy_shuffle: OverworldEnemyShuffle::Vanilla.to_string(),
            pot_shuffle: false,
            shadow_bees: false,
            underworld_balancing: Balancing::Random.to_string(),
            underworld_enemy_shuffle: UnderworldEnemyShuffle::Vanilla.to_string(),
        }
    }
}

pub(crate) fn convert_to_options(value: &OptionsPy) -> Z3Options {
    Z3Options {
        seed: value.seed.clone(),
        boss_shuffle: value.boss_shuffle,
        killable_thieves: value.killable_thieves,
        mushroom_shuffle: value.mushroom_shuffle,
        overworld_balancing: Balancing::from_str(&value.overworld_balancing).unwrap(),
        overworld_enemy_shuffle: OverworldEnemyShuffle::from_str(&value.overworld_enemy_shuffle)
            .unwrap(),
        pot_shuffle: value.pot_shuffle,
        shadow_bees: value.shadow_bees,
        underworld_balancing: Balancing::from_str(&value.underworld_balancing).unwrap(),
        underworld_enemy_shuffle: UnderworldEnemyShuffle::from_str(&value.underworld_enemy_shuffle)
            .unwrap(),
    }
}
