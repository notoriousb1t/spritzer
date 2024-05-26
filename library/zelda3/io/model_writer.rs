use common::SnesGame;
use log;

use crate::zelda3::io::damageclass_writer::write_damage_classes;
use crate::zelda3::io::damagesubclass_writer::write_damage_subclasses;
use crate::zelda3::io::entrance_writer::write_entrances;
use crate::zelda3::io::ow_spritelist_writer::write_ow_sprites_and_headers;
use crate::zelda3::io::sprite_property_writer::write_sprites;
use crate::zelda3::io::spriteset_writer::write_spritesets;
use crate::zelda3::io::uw_header_writer::write_uw_headers;
use crate::zelda3::io::uw_scene_writer::write_uw_scenes;
use crate::zelda3::io::uw_spritelist_writer::write_uw_spritelists;
use crate::zelda3::model::Z3Model;
use crate::zelda3::options::Addresses;

use super::{game_settings_writer::write_game_settings, secret_writer::{write_bush_secrets, write_pot_secrets}};

pub(crate) fn write_model(game: &mut SnesGame, addresses: &Addresses, model: &Z3Model) {
    // Clear all known freespace and fill with 0s.
    game.deallocate();

    // Write all the standard objects back. This needs to happen afterward in case
    // a direct write needs to modify an object before it is written.

    // Note: each of these types as a corresponding writer suffixed module
    // that implements write_objects() for the inferred type.
    write_game_settings(game, addresses, &model.game_settings);
    write_damage_subclasses(game, addresses, &model.damage_subclasses);
    write_damage_classes(game, addresses, &model.damage_classes);
    write_sprites(game, addresses, &model.sprite_settings);
    write_spritesets(game, addresses, &model.spritesets);

    write_ow_sprites_and_headers(game, addresses, &model.ow_rooms);
    write_uw_headers(game, addresses, &model.uw_headers);
    write_uw_spritelists(game, addresses, &model.uw_sprites);
    write_pot_secrets(game, addresses, &model.uw_pot_secrets);
    write_bush_secrets(game, addresses, &model.ow_secrets);
    write_uw_scenes(game, addresses, &model.uw_scenes);
    write_entrances(game, addresses, &model.uw_entrances);

    game.patch(&model.patches);

    log_freespace_report(game);
    // log_banks(game);
}

fn log_freespace_report(game: &SnesGame) {
    let mut freespace_report = "".to_owned();
    for space in game.free_space.iter() {
        freespace_report.push_str(
            format!(
                "\n    Bank ${:02X} has {} remaining",
                space.bank,
                space.capacity()
            )
            .as_str(),
        );
    }
    log::debug!("Capacity:{}", freespace_report);
}

#[allow(dead_code)]
fn log_banks(game: &SnesGame) {
    for bank in 0..0x41 {
        game.print_bank(bank);
    }
}
