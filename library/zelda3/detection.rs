use std::fmt::Display;

use assembly::zelda3::Symbol;
use common::SnesGame;

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
    let game = SnesGame::from_bytes(buffer);
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
        let room_header_bank = game.read(Symbol::UWHeaderBank.into());
        if room_header_bank == 0x36 {
            return GameInfo {
                version: GameVersion::ArchipelagoEnemizer,
                supported: false,
            };
        }

        GameInfo {
            version: GameVersion::Archipelago,
            supported: true,
        }
    } else if title.starts_with("VT") {
        let room_header_bank = game.read(Symbol::UWHeaderBank.into());
        if room_header_bank == 0x36 {
            return GameInfo {
                version: GameVersion::AlttprEnemizer,
                supported: false,
            };
        }

        GameInfo {
            version: GameVersion::Alttpr,
            supported: true,
        }
    } else {
        GameInfo {
            version: GameVersion::UNKNOWN,
            supported: false,
        }
    }
}
