use std::collections::BTreeMap;

use assembly::zelda3::Symbol;
use common::SnesGame;
use strum::IntoEnumIterator;

use super::entrance_utils::BLOCKSET_SIZE;
use super::entrance_utils::CAMERA_SCROLL_BOUNDARIES_SIZE;
use super::entrance_utils::CAMERA_X_TRIGGER_SIZE;
use super::entrance_utils::CAMERA_Y_TRIGGER_SIZE;
use super::entrance_utils::DUNGEON_SIZE;
use super::entrance_utils::ENTRANCE_LEN;
use super::entrance_utils::FLOOR_SIZE;
use super::entrance_utils::H_SCROLL_SIZE;
use super::entrance_utils::INDOOR_SIZE;
use super::entrance_utils::LAYER_SIZE;
use super::entrance_utils::OVERWORLD_TILEMAP;
use super::entrance_utils::QUADRANT_SIZE;
use super::entrance_utils::ROOM_SIZE;
use super::entrance_utils::SCROLL_CAMERA_CONTROLLER_SIZE;
use super::entrance_utils::SONG_SIZE;
use super::entrance_utils::V_SCROLL_SIZE;
use super::entrance_utils::X_COORDINATE_SIZE;
use super::entrance_utils::Y_COORDINATE_SIZE;
use crate::zelda3::model::DungeonId;
use crate::zelda3::model::Entrance;
use crate::zelda3::model::EntranceId;
use crate::zelda3::model::UWBlocksetId;
use crate::zelda3::model::UWRoomId;

pub(super) fn read_entrances(game: &SnesGame) -> BTreeMap<EntranceId, Entrance> {
    BTreeMap::from_iter(EntranceId::iter().map(|id: EntranceId| (id, read_entrance(game, id))))
}

fn read_entrance(game: &SnesGame, entrance_id: EntranceId) -> Entrance {
    let mut cursor = Symbol::Entrances as usize;
    let offset = entrance_id as usize;

    let room_id = game.read_int16(cursor + (offset * ROOM_SIZE));
    cursor += ENTRANCE_LEN * ROOM_SIZE; // Move past Room Ids.
    cursor += ENTRANCE_LEN * CAMERA_SCROLL_BOUNDARIES_SIZE; // Move past Camera Scroll Boundaries.
    cursor += ENTRANCE_LEN * H_SCROLL_SIZE; // Move past Horizontal Scroll.
    cursor += ENTRANCE_LEN * V_SCROLL_SIZE; // Move past Vertical Scroll.
    cursor += ENTRANCE_LEN * Y_COORDINATE_SIZE; // Move past Y Coordinate.
    cursor += ENTRANCE_LEN * X_COORDINATE_SIZE; // Move past X Coordinate.
    cursor += ENTRANCE_LEN * CAMERA_Y_TRIGGER_SIZE; // Move past Camera Y Trigger.
    cursor += ENTRANCE_LEN * CAMERA_X_TRIGGER_SIZE; // Move past Camera X Trigger.

    let blockset_id = game.read(cursor + (offset * BLOCKSET_SIZE));
    cursor += ENTRANCE_LEN * BLOCKSET_SIZE; // Move past BlocksetId.
    cursor += ENTRANCE_LEN * FLOOR_SIZE; // Move past Floor (as in basement).

    let dungeon_id = game.read(cursor + (offset * DUNGEON_SIZE));
    cursor += ENTRANCE_LEN * DUNGEON_SIZE; // Move past Dungeon Ids.
    cursor += ENTRANCE_LEN * INDOOR_SIZE; // Move past "indoor", This is a boolean (mostly true).
    cursor += ENTRANCE_LEN * LAYER_SIZE; // Move past layer.
    cursor += ENTRANCE_LEN * SCROLL_CAMERA_CONTROLLER_SIZE; // Move past camera scroll controller.
    cursor += ENTRANCE_LEN * QUADRANT_SIZE; // Move past quadrant.
    cursor += ENTRANCE_LEN * OVERWORLD_TILEMAP; // Move past overworld door tilemap.

    let song_id = game.read(cursor + (offset * SONG_SIZE));

    Entrance {
        id: entrance_id,
        room_id: UWRoomId::from_repr(room_id).unwrap(),
        blockset_id: UWBlocksetId::from_repr(blockset_id).unwrap(),
        dungeon_id: DungeonId::from_repr(dungeon_id).unwrap(),
        song_id,
    }
}
