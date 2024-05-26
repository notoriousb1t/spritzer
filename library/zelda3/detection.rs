use std::fmt::Display;

use common::SnesGame;

use super::Addresses;

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
    pub(crate) addresses: Addresses,
    pub version: GameVersion,
    pub supported: bool,
}

pub fn detect_game(buffer: &[u8]) -> GameInfo {
    let game = SnesGame::from_bytes(buffer);
    let title = game.get_game_title();

    if title.starts_with("ZELDANODENSETSU") {
        GameInfo {
            addresses: Addresses::for_version(GameVersion::ZeldaJp),
            version: GameVersion::ZeldaJp,
            supported: true,
        }
    } else if title.starts_with("THE LEGEND OF ZELDA") {
        GameInfo {
            addresses: Addresses::for_version(GameVersion::ZeldaUs),
            version: GameVersion::ZeldaUs,
            supported: false,
        }
    } else if title.starts_with("AP") {
        let addresses = Addresses::for_version(GameVersion::Archipelago);
        let room_header_bank = game.read(addresses.uwheader_bank);
        if room_header_bank == 0x36 {
            return GameInfo {
                addresses: Addresses::for_version(GameVersion::ArchipelagoEnemizer),
                version: GameVersion::ArchipelagoEnemizer,
                supported: false,
            };
        }

        GameInfo {
            addresses: Addresses::for_version(GameVersion::Archipelago),
            version: GameVersion::Archipelago,
            supported: true,
        }
    } else if title.starts_with("VT") {
        let addresses = Addresses::for_version(GameVersion::Alttpr);
        let room_header_bank = game.read(addresses.uwheader_bank);
        if room_header_bank == 0x36 {
            return GameInfo {
                addresses: Addresses::for_version(GameVersion::AlttprEnemizer),
                version: GameVersion::AlttprEnemizer,
                supported: false,
            };
        }

        GameInfo {
            addresses: Addresses::for_version(GameVersion::Alttpr),
            version: GameVersion::Alttpr,
            supported: true,
        }
    } else {
        GameInfo {
            addresses: Addresses::for_version(GameVersion::UNKNOWN),
            version: GameVersion::UNKNOWN,
            supported: false,
        }
    }
}
