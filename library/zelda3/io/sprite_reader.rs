use assembly::zelda3::Symbol;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

use crate::zelda3::model::PaletteIndex;
use crate::zelda3::model::Sprite;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::X0_NO_DAMAGE;
use common::SnesGame;

pub(crate) fn read_sprites(game: &SnesGame) -> BTreeMap<SpriteId, Sprite> {
    let mut values: Vec<(SpriteId, Sprite)> = vec![];
    for id in SpriteId::iter() {
        values.push((id, _read_sprite(game, id)));
    }
    BTreeMap::from_iter(values)
}

fn _read_sprite(game: &SnesGame, id: SpriteId) -> Sprite {
    if id as usize >= 0xF3 {
        // Addresses above this value are overlords, this is not applicable.
        return Sprite {
            id,
            allow_pits: false,
            big_shadow: false,
            blockable: false,
            boss_damage_sfx: false,
            boss_death_animation: false,
            boss_prep_preserved: false,
            collision_on_single_layer: false,
            collisions_alt: false,
            custom_death_animation: false,
            deflect_arrows: false,
            die_off_screen: false,
            display_allocation: 0,
            draw_shadow: false,
            harmless: false,
            high_priority: false,
            hitbox: 0,
            hp: 0,
            limit_moving_floor_pit_interaction: false,
            ignore_recoil_collision: false,
            immune_to_sword_hammer: false,
            immune_to_arrows: false,
            immune_to_powder: false,
            impervious: false,
            master_sword_only: false,
            moveable_unused: false,
            name_table: 0,
            palette: PaletteIndex::X0None,
            prevent_permadeath: false,
            projectile: false,
            projectiles_unused: false,
            prize_pack: 0,
            preserved_offscreen: false,
            special_water_check: false,
            slashable: false,
            subclass: X0_NO_DAMAGE,
            statis: false,
            stay_active_offscreen: false,
            tile_hitbox: 0,
        };
    }
    let settings_0_byte = game.read(Symbol::SpriteSettings as usize + id as usize);
    let display_allocation = settings_0_byte & 0b1_1111;
    let collisions_alt = (settings_0_byte & 0b10_0000) != 0;
    let master_sword_only = (settings_0_byte & 0b100_0000) != 0;
    let harmless = (settings_0_byte & 0b1000_0000) != 0;

    // Read from health points table. 0xFF is interpreted as None.
    let hp = game.read(Symbol::SpriteSettings as usize + 0xF3 + id as usize);

    // The damage settings of a sprite
    let settings_2_byte = game.read(Symbol::SpriteSettings as usize + (0xF3 * 2) + id as usize);
    let subclass = settings_2_byte & 0b1111;
    let boss_prep_preserved = (settings_2_byte & 0b1_0000) != 0;
    let immune_to_powder = (settings_2_byte & 0b10_0000) != 0;
    let high_priority = (settings_2_byte & 0b0100_0000) != 0;
    let ignore_recoil_collision = (settings_2_byte & 0b1000_0000) != 0;

    // Draw flags and imperviousness
    let settings_3_byte = game.read(Symbol::SpriteSettings as usize + (0xF3 * 3) + id as usize);
    let name_table = settings_3_byte & 0b1;
    let palette = PaletteIndex::from_repr((settings_3_byte & 0b1110) >> 1).unwrap();
    let draw_shadow: bool = (settings_3_byte & 0b1_0000) != 0;
    let big_shadow: bool = (settings_3_byte & 0b10_0000) != 0;
    let impervious: bool = (settings_3_byte & 0b100_0000) != 0;
    let custom_death_animation: bool = (settings_3_byte & 0b1000_0000) != 0;

    // Collision, kill room, killed off screen, hitbox
    let settings_4_byte = game.read(Symbol::SpriteSettings as usize + (0xF3 * 4) + id as usize);
    let hitbox = settings_4_byte & 0b1_1111;
    let preserved_offscreen = (settings_4_byte & 0b10_0000) != 0;
    let statis = (settings_4_byte & 0b100_0000) != 0;
    let collision_on_single_layer = (settings_4_byte & 0b1000_0000) != 0;

    // Settings group 5
    let settings_5_byte = game.read(Symbol::SpriteSettings as usize + (0xF3 * 5) + id as usize);
    let allow_pits = (settings_5_byte & 0b1) != 0;
    let boss_death_animation = (settings_5_byte & 0b10) != 0;
    let slashable = (settings_5_byte & 0b0100) != 0;
    let deflect_arrows = (settings_5_byte & 0b1000) != 0;
    let tile_hitbox = (settings_5_byte & 0b1111_0000) >> 4;

    // Settings group 6
    let settings_6_byte = game.read(Symbol::SpriteSettings as usize + (0xF3 * 6) + id as usize);
    let prize_pack = settings_6_byte & 0b1111;
    let boss_damage_sfx = (settings_6_byte & 0b1_0000) != 0;
    let blockable = (settings_6_byte & 0b10_0000) != 0;
    let special_water_check = (settings_6_byte & 0b100_0000) != 0;
    let limit_moving_floor_pit_interaction = (settings_6_byte & 0b1000_0000) != 0;

    // Settings group 7
    let settings_7_byte = game.read(Symbol::SpriteSettings as usize + (0xF3 * 7) + id as usize);
    let stay_active_offscreen = (settings_7_byte & 0b1) != 0;
    let die_off_screen = (settings_7_byte & 0b10) != 0;
    let moveable_unused = (settings_7_byte & 0b100) != 0;
    let projectiles_unused = (settings_7_byte & 0b1000) != 0;
    let projectile = (settings_7_byte & 0b1_0000) != 0;
    let immune_to_sword_hammer = (settings_7_byte & 0b10_0000) != 0;
    let immune_to_arrows = (settings_7_byte & 0b100_0000) != 0;
    let prevent_permadeath = (settings_7_byte & 0b1000_0000) != 0;

    Sprite {
        id,
        allow_pits,
        big_shadow,
        blockable,
        boss_damage_sfx,
        boss_death_animation,
        boss_prep_preserved,
        collision_on_single_layer,
        collisions_alt,
        deflect_arrows,
        die_off_screen,
        display_allocation,
        draw_shadow,
        harmless,
        high_priority,
        hitbox,
        hp,
        limit_moving_floor_pit_interaction,
        ignore_recoil_collision,
        immune_to_arrows,
        immune_to_sword_hammer,
        impervious,
        master_sword_only,
        moveable_unused,
        name_table,
        palette,
        preserved_offscreen,
        prevent_permadeath,
        prize_pack,
        projectile,
        projectiles_unused,
        slashable,
        special_water_check,
        statis,
        stay_active_offscreen,
        subclass,
        tile_hitbox,
        custom_death_animation,
        immune_to_powder,
    }
}
