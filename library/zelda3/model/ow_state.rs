use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Hash, EnumIter)]
#[allow(non_camel_case_types)]
pub(crate) enum OWStateId {
    LIGHT_WORLD_V0 = 0,
    LIGHT_WORLD_V1 = 1,
    DARK_WORLD_V1 = 2,
    LIGHT_WORLD_V2 = 3,
    DARK_WORLD_V2 = 4,
}
