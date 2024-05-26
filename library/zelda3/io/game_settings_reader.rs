use common::SnesGame;

use crate::zelda3::model::GameSettings;
use crate::zelda3::Addresses;

pub(super) fn read_game_settings(game: &SnesGame, addresses: &Addresses) -> GameSettings {
    let mut game_settings = GameSettings::default();
    game_settings.is_killable_thief = game.read(addresses.enable_killable_thief) != 0;
    game_settings.moldorm_eye_count = game.read(addresses.moldorm_eye_count) + 1;
    game_settings.is_hera_prize_centered = game.read(addresses.is_hera_prize_centered) != 0;
    game_settings
}
