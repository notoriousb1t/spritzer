use assembly::zelda3::Symbol;
use std::collections::HashMap;

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
use crate::common::readerwriter::WriteObject;
use crate::snes::SnesGame;
use crate::zelda3::model::Entrance;
use crate::zelda3::model::EntranceId;

impl WriteObject<HashMap<EntranceId, Entrance>> for SnesGame {
    fn write_objects(&mut self, entrances: &HashMap<EntranceId, Entrance>) {
        let mut ids = entrances.keys().collect::<Vec<_>>();
        ids.sort();

        for id in ids {
            let entrance = entrances.get(id).unwrap();
            write_entrance(self, entrance);
        }
    }
}

fn write_entrance(game: &mut SnesGame, entrance: &Entrance) {
    let mut cursor = Symbol::Entrances as usize;
    let offset = entrance.id as usize;

    game.write_int16(cursor + (offset * ROOM_SIZE), entrance.room_id as u16);
    cursor += ENTRANCE_LEN * ROOM_SIZE; // Move past Room Ids.
    cursor += ENTRANCE_LEN * CAMERA_SCROLL_BOUNDARIES_SIZE; // Move past Camera Scroll Boundaries.
    cursor += ENTRANCE_LEN * H_SCROLL_SIZE; // Move past Horizontal Scroll.
    cursor += ENTRANCE_LEN * V_SCROLL_SIZE; // Move past Vertical Scroll.
    cursor += ENTRANCE_LEN * Y_COORDINATE_SIZE; // Move past Y Coordinate.
    cursor += ENTRANCE_LEN * X_COORDINATE_SIZE; // Move past X Coordinate.
    cursor += ENTRANCE_LEN * CAMERA_Y_TRIGGER_SIZE; // Move past Camera Y Trigger.
    cursor += ENTRANCE_LEN * CAMERA_X_TRIGGER_SIZE; // Move past Camera X Trigger.

    game.write(
        cursor + (offset * BLOCKSET_SIZE),
        entrance.blockset_id as u8,
    );
    cursor += ENTRANCE_LEN * BLOCKSET_SIZE; // Move past BlocksetId.
    cursor += ENTRANCE_LEN * FLOOR_SIZE; // Move past Floor (as in basement).

    game.write(cursor + (offset * DUNGEON_SIZE), entrance.dungeon_id as u8);
    cursor += ENTRANCE_LEN * DUNGEON_SIZE; // Move past Dungeon Ids.
    cursor += ENTRANCE_LEN * INDOOR_SIZE; // Move past "indoor", This is a boolean (mostly true).
    cursor += ENTRANCE_LEN * LAYER_SIZE; // Move past layer.
    cursor += ENTRANCE_LEN * SCROLL_CAMERA_CONTROLLER_SIZE; // Move past camera scroll controller.
    cursor += ENTRANCE_LEN * QUADRANT_SIZE; // Move past quadrant.
    cursor += ENTRANCE_LEN * OVERWORLD_TILEMAP; // Move past overworld door tilemap.

    game.write(cursor + (offset * SONG_SIZE), entrance.song_id);
}
