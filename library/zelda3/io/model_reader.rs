use common::SnesGame;

use super::damageclass_reader::read_damage_classes;
use super::damagesubclass_reader::read_damage_subclasses;
use super::dungeon_reader::read_dungeons;
use super::entrance_reader::read_entrances;
use super::ow_spritelist_reader::read_ow_sprites_and_headers;
use super::sprite_reader::read_sprites;
use super::spriteset_reader::read_spritesets;
use super::uw_header_reader::read_uw_headers;
use super::uw_scene_reader::read_uw_scenes;
use super::uw_spritelist_reader::read_uw_spritelists;
use crate::zelda3::model::Z3Model;

pub(crate) fn read_model(game: &SnesGame) -> Z3Model {
    // Create a new model and read all data types to their respective names.
    let mut model = Z3Model::default();
    // Note: each of these types as a corresponding reader suffixed module
    // that implements read_objects() for the inferred type.
    model.damage_classes = read_damage_classes(game);
    model.damage_subclasses = read_damage_subclasses(game);
    model.ow_rooms = read_ow_sprites_and_headers(game);
    model.sprite_settings = read_sprites(game);
    model.spritesets = read_spritesets(game);
    model.dungeons = read_dungeons();
    model.uw_scenes = read_uw_scenes(game);
    model.uw_headers = read_uw_headers(game);
    model.uw_sprites = read_uw_spritelists(game);
    model.uw_entrances = read_entrances(game);
    model
}
