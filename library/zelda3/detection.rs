use std::fmt::Display;

use crate::snes::SnesGame;

pub enum GameVersion {
    UNKNOWN = 0,
    ZeldaJp = 1,
    ZeldaUs = 2,
    Archipelago = 10,
    Alttpr = 20,
    ArchipelagoEnemizer = 11,
    AlttprEnemizer = 21,
}

impl Display for GameVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameVersion::ZeldaJp => "Zelda JP 1.0",
                GameVersion::ZeldaUs => "Zelda US",
                GameVersion::Archipelago => "Archipelago",
                GameVersion::ArchipelagoEnemizer => "Archipelago + Enemizer",
                GameVersion::Alttpr => "A Link to the Past Randomizer",
                GameVersion::AlttprEnemizer => "A Link to the Past Randomizer Enemizer",
                _ => "Unknown Game",
            }
        )
    }
}

pub struct GameInfo {
    pub version: GameVersion,
    pub supported: bool,
}

pub fn detect_game(buffer: &[u8]) -> GameInfo {
    let game = SnesGame::new(buffer);
    let title = game.get_game_title();

    if title.starts_with("ZELDANODENSETSU") {
        GameInfo {
            version: GameVersion::ZeldaJp,
            supported: false,
        }
    } else if title.starts_with("THE LEGEND OF ZELDA") {
        GameInfo {
            version: GameVersion::ZeldaUs,
            supported: false,
        }
    } else if title.starts_with("AP") {
        GameInfo {
            version: GameVersion::Archipelago,
            supported: true,
        }
    } else if title.starts_with("VT") {
        GameInfo {
            version: GameVersion::Alttpr,
            supported: false,
        }
    } else {
        GameInfo {
            version: GameVersion::UNKNOWN,
            supported: false,
        }
    }
}
