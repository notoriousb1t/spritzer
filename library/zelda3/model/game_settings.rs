/// Contains settings applied in game.
#[derive(Clone)]
pub(crate) struct GameSettings {
    pub(crate) is_killable_thief: bool,
    pub(crate) is_hera_prize_centered: bool,
    pub(crate) moldorm_eye_count: u8,
}

impl GameSettings {
    pub(crate) fn default() -> GameSettings {
        GameSettings {
            is_killable_thief: false,
            is_hera_prize_centered: true,
            moldorm_eye_count: 2,
        }
    }
}