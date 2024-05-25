use common::SnesGame;

use crate::zelda3::model::DamageSubclass;
use crate::zelda3::options::Addresses;

/// SNES 0x36F33 - Damage ptr of weapons - 4 bytes (also determines which enemies you can kill
/// with, and which enemies will be one-hit-ko)
/// NOTE: this doesn't seem to work with Archipelago (probably intercepting damage subroutine).
pub(super) fn read_damage_subclasses(game: &SnesGame, addresses: &Addresses) -> [DamageSubclass; 10] {
    let mut subclass_table: [DamageSubclass; 10] = [DamageSubclass::default(); 10];
    for index in 0..10 {
        let row = subclass_table.get_mut(index).unwrap();
        row.boomerang = game.read(addresses.damage_subclass + (index * 8));
        row.sword1 = game.read(addresses.damage_subclass + (index * 8) + 0x1);
        row.sword2 = game.read(addresses.damage_subclass + (index * 8) + 0x2);
        row.sword3 = game.read(addresses.damage_subclass + (index * 8) + 0x3);
        row.sword4 = game.read(addresses.damage_subclass + (index * 8) + 0x4);
        row.sword5 = game.read(addresses.damage_subclass + (index * 8) + 0x5);
        row.arrow1 = game.read(addresses.damage_subclass + (index * 8) + 0x6);
        row.hookshot = game.read(addresses.damage_subclass + (index * 8) + 0x7);
        row.bomb = game.read(addresses.damage_subclass + (index * 8) + 0x8);
        row.arrow2 = game.read(addresses.damage_subclass + (index * 8) + 0x9);
        row.powder = game.read(addresses.damage_subclass + (index * 8) + 0xA);
        row.fire_rod = game.read(addresses.damage_subclass + (index * 8) + 0xB);
        row.ice_rod = game.read(addresses.damage_subclass + (index * 8) + 0xC);
        row.bombos = game.read(addresses.damage_subclass + (index * 8) + 0xD);
        row.ether = game.read(addresses.damage_subclass + (index * 8) + 0xE);
        row.quake = game.read(addresses.damage_subclass + (index * 8) + 0xF);
    }
    subclass_table
}
