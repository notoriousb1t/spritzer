use strum_macros::Display;
use strum_macros::EnumIter;

use super::Rule;

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Hash, EnumIter, Ord, PartialOrd)]
#[allow(non_camel_case_types)]
pub(crate) enum OWStateId {
    LIGHT_WORLD_V0 = 0,
    LIGHT_WORLD_V1 = 1,
    DARK_WORLD_V1 = 2,
    LIGHT_WORLD_V2 = 3,
    DARK_WORLD_V2 = 4,
}

/// Returns the sprite rules that apply for this area.
pub(crate) fn get_overworld_rules(overworld_id: OWStateId) -> Vec<Rule> {
    match overworld_id {
        OWStateId::LIGHT_WORLD_V0 => {
            vec![Rule::Overworld, Rule::LightWorld, Rule::ReduceDifficulty]
        }
        OWStateId::LIGHT_WORLD_V1 => vec![Rule::Overworld, Rule::LightWorld],
        OWStateId::LIGHT_WORLD_V2 => vec![Rule::Overworld, Rule::LightWorld],
        OWStateId::DARK_WORLD_V1 => vec![Rule::Overworld, Rule::DarkWorld],
        OWStateId::DARK_WORLD_V2 => vec![Rule::Overworld, Rule::DarkWorld],
    }
}
