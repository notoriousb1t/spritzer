use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::SeedableRng;

use super::calculate_sprite_pool;
use super::DamageClass;
use super::DamageSubclass;
use super::Dungeon;
use super::DungeonId;
use super::Entrance;
use super::EntranceId;
use super::OWRoom;
use super::OWRoomId;
use super::Sprite;
use super::SpriteId;
use super::Spriteset;
use super::SpritesetId;
use super::UWRoomId;
use super::UWScene;
use super::UWSpriteList;
use super::UnderworldRoomHeader;
use crate::snes::Patch;
use crate::zelda3::options::Balancing;

#[derive(Clone)]
pub(crate) struct Z3Model {
    /// The seed hash.
    pub seed: u64,
    /// Used to manually override sections for debugging purpose. Do not use this to deliver
    /// features.
    pub debug_string: String,
    /// Contains the main damage table for computing damaged against Link.
    pub damage_classes: [DamageClass; 10],
    /// The amount of damage each weapon type creates at base value.
    pub damage_subclasses: [DamageSubclass; 10],
    /// A lookup of associated underworld areas.
    pub dungeons: HashMap<DungeonId, Dungeon>,
    /// The general difficulty of randomization of sprites in the overworld
    pub ow_balancing: Balancing,
    /// The current set of areas in the overworld map
    pub ow_rooms: HashMap<OWRoomId, OWRoom>,
    /// A list of remaining spritesets that can be mapped to overworld sprite sheets
    pub ow_spritesets_unused: Vec<SpritesetId>,
    /// The list of patches to apply after write all other data. This is typically where direct
    /// address replacements should be provided when there isn't a corresponding object in the
    /// rest of the model to describe the desired change.
    pub patches: Vec<Patch>,
    /// The association between spritesets (graphics) and the sprites that are available there.
    pub sprite_pool: HashMap<SpritesetId, Vec<SpriteId>>,
    /// The Settings of each Sprite. Please note that the game logic may have hard coded values as
    /// well.
    pub sprite_settings: HashMap<SpriteId, Sprite>,
    /// The Spriteset blocks used to rendered sprites. There are 4 associated with each ID.
    pub spritesets: HashMap<SpritesetId, Spriteset>,
    /// The general difficulty of randomization of sprites in the underworld
    pub uw_balancing: Balancing,
    /// Contains all known entrances.
    pub uw_entrances: HashMap<EntranceId, Entrance>,
    /// The general metadata about a room.
    pub uw_headers: HashMap<UWRoomId, UnderworldRoomHeader>,
    /// A list of remaining spritesets that can be mapped to underworld sprite sheets
    pub uw_spritesets_unused: Vec<SpritesetId>,
    /// The current set of rooms in caves, dungeons, etc. in the underworld
    pub uw_sprites: HashMap<UWRoomId, UWSpriteList>,
    /// The layouts and doors associated with an underworld room.
    pub uw_scenes: HashMap<UWRoomId, UWScene>,
}

impl Z3Model {
    pub(crate) fn default() -> Z3Model {
        Z3Model {
            seed: 0,
            debug_string: "".to_owned(),
            damage_classes: [DamageClass::default(); 10],
            damage_subclasses: [DamageSubclass::default(); 10],
            dungeons: HashMap::default(),
            ow_balancing: Balancing::Random,
            ow_rooms: HashMap::default(),
            ow_spritesets_unused: create_free_ow_spriteset_list(),
            patches: Vec::new(),
            sprite_pool: HashMap::default(),
            sprite_settings: HashMap::default(),
            spritesets: HashMap::default(),
            uw_balancing: Balancing::Random,
            uw_entrances: HashMap::default(),
            uw_headers: HashMap::default(),
            uw_scenes: HashMap::default(),
            uw_sprites: HashMap::default(),
            uw_spritesets_unused: create_free_uw_spriteset_list(),
        }
    }

    pub(crate) fn create_rng(&self) -> StdRng {
        StdRng::seed_from_u64(self.seed)
    }

    /// This must be called before asking which Sprites are available.
    pub(crate) fn prepare_sprite_pool(&mut self) {
        self.sprite_pool = calculate_sprite_pool(self);
    }
}

/// Returns a list of spritesheets that are considered empty and can be used for swapping.
fn create_free_ow_spriteset_list() -> Vec<SpritesetId> {
    vec![
        SpritesetId::x2F_FREESPACE,
        SpritesetId::x30_FREESPACE,
        SpritesetId::x31_FREESPACE,
        SpritesetId::x32_FREESPACE,
        SpritesetId::x33_FREESPACE,
        SpritesetId::x34_FREESPACE,
        SpritesetId::x35_FREESPACE,
        SpritesetId::x36_FREESPACE,
        SpritesetId::x37_FREESPACE,
        SpritesetId::x38_FREESPACE,
        SpritesetId::x39_FREESPACE,
        SpritesetId::x3A_FREESPACE,
        SpritesetId::x3B_FREESPACE,
        SpritesetId::x3C_FREESPACE,
        SpritesetId::x3D_FREESPACE,
        SpritesetId::x3E_FREESPACE,
        SpritesetId::x3F_FREESPACE,
        SpritesetId::x40_FREESPACE,
    ]
}

/// Returns a list of spritesheets that are considered empty and can be used for swapping.
fn create_free_uw_spriteset_list() -> Vec<SpritesetId> {
    vec![
        SpritesetId::x2C_UNDERWORLD_FREESPACE,
        SpritesetId::x2D_UNDERWORLD_FREESPACE,
        SpritesetId::x2E_UNDERWORLD_FREESPACE,
        SpritesetId::x3F_UNDERWORLD_FREESPACE,
        SpritesetId::x30_UNDERWORLD_FREESPACE,
        SpritesetId::x31_UNDERWORLD_FREESPACE,
        SpritesetId::x32_UNDERWORLD_FREESPACE,
        SpritesetId::x33_UNDERWORLD_FREESPACE,
        SpritesetId::x34_UNDERWORLD_FREESPACE,
        SpritesetId::x35_UNDERWORLD_FREESPACE,
        SpritesetId::x36_UNDERWORLD_FREESPACE,
        SpritesetId::x37_UNDERWORLD_FREESPACE,
        SpritesetId::x38_UNDERWORLD_FREESPACE,
        SpritesetId::x39_UNDERWORLD_FREESPACE,
        SpritesetId::x3A_UNDERWORLD_FREESPACE,
        SpritesetId::x3B_UNDERWORLD_FREESPACE,
        SpritesetId::x3C_UNDERWORLD_FREESPACE,
    ]
}
