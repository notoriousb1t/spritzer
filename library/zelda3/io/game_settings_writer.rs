use common::SnesGame;

use crate::zelda3::{model::GameSettings, Addresses};

pub(super) fn write_game_settings(
    game: &mut SnesGame,
    addresses: &Addresses,
    game_settings: &GameSettings,
) {
    game.write(
        addresses.enable_killable_thief,
        game_settings.is_killable_thief.into(),
    );
    game.write(
        addresses.moldorm_eye_count,
        game_settings.moldorm_eye_count - 1,
    );
    game.write(
        addresses.is_hera_prize_centered,
        game_settings.is_hera_prize_centered.into(),
    );
}
