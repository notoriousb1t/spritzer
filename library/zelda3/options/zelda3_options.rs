use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::FromRepr;

#[repr(i32)]
#[derive(
    Clone, Copy, PartialEq, PartialOrd, Debug, Display, Eq, EnumString, EnumIter, FromRepr,
)]
pub enum Balancing {
    Random = 0,
    Casual = 1,
    Balanced = 2,
    Hero = 3,
}

impl Balancing {
    // iter() is not working outside of the crate, so just export a collection.
    pub fn all() -> Vec<Self> {
        Self::iter().collect::<Vec<_>>()
    }
}

#[repr(i32)]
#[derive(
    Clone, Copy, PartialEq, PartialOrd, Debug, Display, Eq, EnumString, EnumIter, FromRepr,
)]
pub enum OverworldEnemyShuffle {
    Vanilla = 0,
    Full = 1,
    Chaos = 2,
    Insanity = 3,
}

impl OverworldEnemyShuffle {
    // iter() is not working outside of the crate, so just export a collection.
    pub fn all() -> Vec<Self> {
        Self::iter().collect::<Vec<_>>()
    }
}

#[repr(i32)]
#[derive(
    Clone, Copy, PartialEq, PartialOrd, Debug, Display, Eq, EnumString, EnumIter, FromRepr,
)]
pub enum UnderworldEnemyShuffle {
    Vanilla = 0,
    Full = 1,
    Chaos = 2,
    Insanity = 3,
}

impl UnderworldEnemyShuffle {
    // iter() is not working outside of the crate, so just export a collection.
    pub fn all() -> Vec<Self> {
        Self::iter().collect::<Vec<_>>()
    }
}

#[derive(Clone)]
pub struct Z3Options {
    pub seed: String,
    pub boss_shuffle: bool,
    pub mushroom_shuffle: bool,
    pub killable_thieves: bool,
    pub overworld_inverted: bool,
    pub overworld_balancing: Balancing,
    pub overworld_enemy_shuffle: OverworldEnemyShuffle,
    pub pot_shuffle: bool,
    pub shadow_bees: bool,
    pub underworld_balancing: Balancing,
    pub underworld_enemy_shuffle: UnderworldEnemyShuffle,
}

impl Z3Options {
    #[allow(dead_code)]
    pub(crate) fn default() -> Self {
        Z3Options {
            seed: "".to_owned(),
            boss_shuffle: false,
            mushroom_shuffle: false,
            killable_thieves: false,
            pot_shuffle: false,
            overworld_balancing: Balancing::Random,
            overworld_inverted: false,
            overworld_enemy_shuffle: OverworldEnemyShuffle::Vanilla,
            shadow_bees: false,
            underworld_balancing: Balancing::Random,
            underworld_enemy_shuffle: UnderworldEnemyShuffle::Vanilla,
        }
    }
}

impl Display for Z3Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Options:
  Seed:                 {}
  Boss Shuffle:         {}
  Killable Thieves:     {}
  Mushroom Shuffle:     {}
  Overworld Balancing:  {}
  Overworld Shuffle:    {}
  Shadow Bees:          {}
  Underworld Balancing: {}
  Underworld Shuffle:   {}
"#,
            self.seed,
            self.boss_shuffle,
            self.killable_thieves,
            self.mushroom_shuffle,
            self.overworld_balancing,
            self.overworld_enemy_shuffle,
            self.shadow_bees,
            self.underworld_balancing,
            self.underworld_enemy_shuffle,
        )
    }
}
