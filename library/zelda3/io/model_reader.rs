use common::SnesGame;

use super::damageclass_reader::read_damage_classes;
use super::damagesubclass_reader::read_damage_subclasses;
use super::dungeon_reader::read_dungeons;
use super::entrance_reader::read_entrances;
use super::game_settings_reader::read_game_settings;
use super::ow_spritelist_reader::read_ow_sprites_and_headers;
use super::pit_damage_reader::read_pit_damage;
use super::sprite_property_reader::read_sprites;
use super::spriteset_reader::read_spritesets;
use super::uw_header_reader::read_uw_headers;
use super::uw_scene_reader::read_uw_scenes;
use super::uw_spritelist_reader::read_uw_spritelists;
use super::secret_reader::{read_bush_secrets, read_pot_secrets};
use crate::zelda3::model::Z3Model;
use crate::zelda3::options::Addresses;

pub(crate) fn read_model(game: &SnesGame, addresses: &Addresses) -> Z3Model {
    // Create a new model and read all data types to their respective names.
    let mut model = Z3Model::default();
    // Note: each of these types as a corresponding reader suffixed module
    // that implements read_objects() for the inferred type.
    model.game_settings = read_game_settings(game, addresses);
    model.damage_classes = read_damage_classes(game, addresses);
    model.damage_subclasses = read_damage_subclasses(game, addresses);
    model.ow_rooms = read_ow_sprites_and_headers(game, addresses);
    model.sprite_settings = read_sprites(game, addresses);
    model.spritesets = read_spritesets(game, addresses);
    model.dungeons = read_dungeons();
    model.uw_scenes = read_uw_scenes(game, addresses);
    model.uw_headers = read_uw_headers(game, addresses);
    model.uw_sprites = read_uw_spritelists(game, addresses);
    model.uw_entrances = read_entrances(game, addresses);
    model.ow_secrets = read_bush_secrets(game, addresses);
    model.uw_pot_secrets = read_pot_secrets(game, addresses);
    model.pit_damage = read_pit_damage(game, addresses);
    model
}
