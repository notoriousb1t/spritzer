use common::SnesGame;

use crate::zelda3::model::DamageSubclass;
use crate::zelda3::options::Addresses;

/// SNES 0x36F33 - Damage ptr of weapons - 4 bytes (also determines which enemies you can kill
/// with, and which enemies will be one-hit-ko)
/// NOTE: this doesn't seem to work with Archipelago (probably intercepting damage subroutine).
pub(super) fn read_damage_subclasses(game: &SnesGame, addresses: &Addresses) -> [DamageSubclass; 8] {
    let bytes = game.read_all::<128>(addresses.damage_subclass);
    bytes_to_damage_subclass_table(bytes)
}

fn bytes_to_damage_subclass_table(bytes: [u8; 128]) -> [DamageSubclass; 8] {
    let mut subclass_table = [DamageSubclass::default(); 8];
    for i in 0..8 {
        subclass_table[i].boomerang = bytes[(i * 8) + 0];
        subclass_table[i].sword1 = bytes[(i * 8) + 1];
        subclass_table[i].sword2 = bytes[(i * 8) + 2];
        subclass_table[i].sword3 = bytes[(i * 8) + 3];
        subclass_table[i].sword4 = bytes[(i * 8) + 4];
        subclass_table[i].sword5 = bytes[(i * 8) + 5];
        subclass_table[i].arrow1 = bytes[(i * 8) + 6];
        subclass_table[i].hookshot = bytes[(i * 8) + 7];
        subclass_table[i].bomb = bytes[(i * 8) + 8];
        subclass_table[i].arrow2 = bytes[(i * 8) + 9];
        subclass_table[i].powder = bytes[(i * 8) + 10];
        subclass_table[i].fire_rod = bytes[(i * 8) + 11];
        subclass_table[i].ice_rod = bytes[(i * 8) + 12];
        subclass_table[i].bombos = bytes[(i * 8) + 13];
        subclass_table[i].ether = bytes[(i * 8) + 14];
        subclass_table[i].quake = bytes[(i * 8) + 15];
    }
    subclass_table
}