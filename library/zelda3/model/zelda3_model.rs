use std::collections::BTreeMap;

use common::Patch;
use rand::rngs::StdRng;
use rand::SeedableRng;

use super::calculate_sprite_pool;
use super::game_settings::GameSettings;
use super::DamageClass;
use super::DamageSubclass;
use super::Dungeon;
use super::DungeonId;
use super::Entrance;
use super::EntranceId;
use super::OWRoom;
use super::OWRoomId;
use super::OWSecrets;
use super::SpriteId;
use super::SpriteProperties;
use super::Spriteset;
use super::SpritesetId;
use super::UWRoomId;
use super::UWScene;
use super::UWSpriteList;
use super::UnderworldRoomHeader;
use super::Secret;
use crate::zelda3::options::Balancing;

#[derive(Clone)]
pub(crate) struct Z3Model {
    /// The seed hash.
    pub seed: u64,
    pub ow_secrets: BTreeMap<OWRoomId, OWSecrets>,
    /// Used to manually override sections for debugging purpose. Do not use this to deliver
    /// features.
    pub debug_string: String,
    /// Contains the main damage table for computing damaged against Link.
    pub damage_classes: [DamageClass; 10],
    /// The amount of damage each weapon type creates at base value.
    pub damage_subclasses: [DamageSubclass; 10],
    /// A lookup of associated underworld areas.
    pub dungeons: BTreeMap<DungeonId, Dungeon>,
    /// The general difficulty of randomization of sprites in the overworld
    pub ow_balancing: Balancing,
    /// The current set of areas in the overworld map
    pub ow_rooms: BTreeMap<OWRoomId, OWRoom>,
    /// A list of remaining spritesets that can be mapped to overworld sprite sheets
    pub ow_spritesets_unused: Vec<SpritesetId>,
    /// The list of patches to apply after write all other data. This is typically where direct
    /// address replacements should be provided when there isn't a corresponding object in the
    /// rest of the model to describe the desired change.
    pub patches: Vec<Patch>,
    /// The association between spritesets (graphics) and the sprites that are available there.
    pub sprite_pool: BTreeMap<SpritesetId, Vec<SpriteId>>,
    /// The Settings of each Sprite. Please note that the game logic may have hard coded values as
    /// well.
    pub sprite_settings: BTreeMap<SpriteId, SpriteProperties>,
    /// The Spriteset blocks used to rendered sprites. There are 4 associated with each ID.
    pub spritesets: BTreeMap<SpritesetId, Spriteset>,
    /// The general difficulty of randomization of sprites in the underworld
    pub uw_balancing: Balancing,
    /// Contains all known entrances.
    pub uw_entrances: BTreeMap<EntranceId, Entrance>,
    /// The general metadata about a room.
    pub uw_headers: BTreeMap<UWRoomId, UnderworldRoomHeader>,
    /// The pot items contained in an underground room.
    pub uw_pot_secrets: BTreeMap<UWRoomId, Vec<Secret>>,
    /// A list of remaining spritesets that can be mapped to underworld sprite sheets
    pub uw_spritesets_unused: Vec<SpritesetId>,
    /// The current set of rooms in caves, dungeons, etc. in the underworld
    pub uw_sprites: BTreeMap<UWRoomId, UWSpriteList>,
    /// The layouts and doors associated with an underworld room.
    pub uw_scenes: BTreeMap<UWRoomId, UWScene>,
    /// In game settings applied to the binary.
    pub game_settings: GameSettings,
}

impl Z3Model {
    pub(crate) fn default() -> Z3Model {
        Z3Model {
            seed: 0,
            debug_string: "".to_owned(),
            damage_classes: [DamageClass::default(); 10],
            damage_subclasses: [DamageSubclass::default(); 10],
            dungeons: BTreeMap::default(),
            ow_balancing: Balancing::Random,
            ow_secrets: BTreeMap::default(),
            ow_rooms: BTreeMap::default(),
            ow_spritesets_unused: create_free_ow_spriteset_list(),
            patches: Vec::new(),
            sprite_pool: BTreeMap::default(),
            sprite_settings: BTreeMap::default(),
            spritesets: BTreeMap::default(),
            uw_balancing: Balancing::Random,
            uw_entrances: BTreeMap::default(),
            uw_headers: BTreeMap::default(),
            uw_pot_secrets: BTreeMap::default(),
            uw_scenes: BTreeMap::default(),
            uw_sprites: BTreeMap::default(),
            uw_spritesets_unused: create_free_uw_spriteset_list(),
            game_settings: GameSettings::default(),
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
    ]
}

/// Returns a list of spritesheets that are considered empty and can be used for swapping.
fn create_free_uw_spriteset_list() -> Vec<SpritesetId> {
    vec![
        SpritesetId::x6C_UNDERWORLD_FREESPACE,
        SpritesetId::x6D_UNDERWORLD_FREESPACE,
        SpritesetId::x6E_UNDERWORLD_FREESPACE,
        SpritesetId::x6F_UNDERWORLD_FREESPACE,
        SpritesetId::x70_UNDERWORLD_FREESPACE,
        SpritesetId::x71_UNDERWORLD_FREESPACE,
        SpritesetId::x72_UNDERWORLD_FREESPACE,
        SpritesetId::x73_UNDERWORLD_FREESPACE,
        SpritesetId::x74_UNDERWORLD_FREESPACE,
        SpritesetId::x75_UNDERWORLD_FREESPACE,
        SpritesetId::x76_UNDERWORLD_FREESPACE,
        SpritesetId::x77_UNDERWORLD_FREESPACE,
        SpritesetId::x78_UNDERWORLD_FREESPACE,
        SpritesetId::x79_UNDERWORLD_FREESPACE,
        SpritesetId::x7A_UNDERWORLD_FREESPACE,
        SpritesetId::x7B_UNDERWORLD_FREESPACE,
        SpritesetId::x7C_UNDERWORLD_FREESPACE,
    ]
}
