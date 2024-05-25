use std::collections::BTreeMap;

use common::SnesGame;

use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteProperties;
use crate::zelda3::Addresses;

pub(super) fn write_sprites(
    game: &mut SnesGame,
    addresses: &Addresses,
    sprites: &BTreeMap<SpriteId, SpriteProperties>,
) {
    for (id, sprite) in sprites.iter() {
        if *id as usize >= 0xF3 {
            // Skip overlords, they aren't real Sprites.
            continue;
        }

        game.write(
            addresses.sprite_settings + *id as usize,
            (sprite.display_allocation)
                | (if sprite.collisions_alt { 0b10_0000 } else { 0 })
                | (if sprite.master_sword_only {
                    0b100_0000
                } else {
                    0
                })
                | (if sprite.harmless { 0b1000_0000 } else { 0 }),
        );
        game.write(addresses.sprite_settings + 0xF3 + *id as usize, sprite.hp);
        game.write(
            addresses.sprite_settings + (0xF3 * 2) + *id as usize,
            sprite.subclass
                | (if sprite.boss_prep_preserved {
                    0b1_0000
                } else {
                    0
                })
                | (if sprite.immune_to_powder {
                    0b10_0000
                } else {
                    0
                })
                | (if sprite.high_priority { 0b100_0000 } else { 0 })
                | (if sprite.ignore_recoil_collision {
                    0b1000_0000
                } else {
                    0
                }),
        );
        game.write(
            addresses.sprite_settings + (0xF3 * 3) + *id as usize,
            (sprite.name_table)
                | ((sprite.palette as u8) << 1)
                | (if sprite.draw_shadow { 0b1_0000 } else { 0 })
                | (if sprite.big_shadow { 0b10_0000 } else { 0 })
                | (if sprite.impervious { 0b100_0000 } else { 0 })
                | (if sprite.custom_death_animation {
                    0b1000_0000
                } else {
                    0
                }),
        );
        game.write(
            addresses.sprite_settings + (0xF3 * 4) + *id as usize,
            (sprite.hitbox & 0b11111)
                | (if sprite.preserved_offscreen {
                    0b10_0000
                } else {
                    0
                })
                | (if sprite.statis { 0b100_0000 } else { 0 })
                | (if sprite.collision_on_single_layer {
                    0b1000_0000
                } else {
                    0
                }),
        );
        game.write(
            addresses.sprite_settings + (0xF3 * 5) + *id as usize,
            (if sprite.allow_pits { 0b1 } else { 0 })
                | (if sprite.boss_death_animation { 0b10 } else { 0 })
                | (if sprite.slashable { 0b100 } else { 0 })
                | (if sprite.deflect_arrows { 0b1000 } else { 0 })
                | ((sprite.tile_hitbox & 0b1111) << 4),
        );
        game.write(
            addresses.sprite_settings + (0xF3 * 6) + *id as usize,
            (sprite.prize_pack & 0b1111)
                | (if sprite.boss_damage_sfx { 0b1_0000 } else { 0 })
                | (if sprite.blockable { 0b10_0000 } else { 0 })
                | (if sprite.special_water_check {
                    0b100_0000
                } else {
                    0
                })
                | (if sprite.limit_moving_floor_pit_interaction {
                    0b1000_0000
                } else {
                    0
                }),
        );
        game.write(
            addresses.sprite_settings + (0xF3 * 7) + *id as usize,
            (if sprite.stay_active_offscreen { 0b1 } else { 0 })
                | (if sprite.die_off_screen { 0b10 } else { 0 })
                | (if sprite.moveable_unused { 0b100 } else { 0 })
                | (if sprite.projectiles_unused { 0b1000 } else { 0 })
                | (if sprite.projectile { 0b1_0000 } else { 0 })
                | (if sprite.immune_to_sword_hammer {
                    0b10_0000
                } else {
                    0
                })
                | (if sprite.immune_to_arrows {
                    0b100_0000
                } else {
                    0
                })
                | (if sprite.prevent_permadeath {
                    0b1000_0000
                } else {
                    0
                }),
        );
    }
}
