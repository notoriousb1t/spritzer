use options::convert_to_options;
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::types::PyBytes;
use simplelog::ColorChoice;
use simplelog::CombinedLogger;
use simplelog::Config;
use simplelog::LevelFilter;
use simplelog::TermLogger;
use simplelog::TerminalMode;
use ::spritzer::zelda3::randomize_zelda3;
use ::spritzer::zelda3::Balancing;
use ::spritzer::zelda3::OverworldEnemyShuffle;
use ::spritzer::zelda3::UnderworldEnemyShuffle;
use strum::IntoEnumIterator;

use crate::options::OptionsPy;

mod options;

#[pyfunction]
fn list_balancing_options() -> Vec<String> {
    Balancing::iter()
        .map(|it| it.to_string())
        .collect::<Vec<_>>()
}

#[pyfunction]
fn list_overworld_enemy_shuffle_options() -> Vec<String> {
    OverworldEnemyShuffle::iter()
        .map(|it| it.to_string())
        .collect::<Vec<_>>()
}

#[pyfunction]
fn list_underworld_enemy_shuffle_options() -> Vec<String> {
    UnderworldEnemyShuffle::iter()
        .map(|it| it.to_string())
        .collect::<Vec<_>>()
}

#[pyfunction]
#[pyo3(name = "randomize_zelda3")]
/// Binds to the main function of Spritzer.
fn randomize_zelda3_py<'a>(
    py: Python<'a>,
    bytes: &PyByteArray,
    spritzer_options: &OptionsPy,
) -> &'a PyBytes {
    let z3options = convert_to_options(spritzer_options);
    let data = randomize_zelda3(&bytes.to_vec(), &z3options);
    PyBytes::new(py, &data)
}

#[pyfunction]
#[pyo3(name = "init")]
/// Call to setup logging, etc.
fn init() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}

#[pymodule]
#[pyo3(name = "spritzer_py")]
/// Configures the exported symbols from the Python module.
fn spritzer(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();

    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(list_balancing_options, m)?)?;
    m.add_function(wrap_pyfunction!(list_overworld_enemy_shuffle_options, m)?)?;
    m.add_function(wrap_pyfunction!(list_underworld_enemy_shuffle_options, m)?)?;
    m.add_function(wrap_pyfunction!(randomize_zelda3_py, m)?)?;
    m.add_class::<OptionsPy>()?;
    Ok(())
}
