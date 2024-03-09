//! This module contains match expressions for attributes about sprites known
//! to this randomizer which are not part of game itself. This is used to
//! determine randomization behavior, and is subjective.

use super::sprite_id::SpriteId;
use super::sprite_sheet_id::SpriteSheetId;
use super::sprite_type::SpriteType;
use crate::zelda3::model::DIAGONAL;
use crate::zelda3::model::EAST;
use crate::zelda3::model::FIXED;
use crate::zelda3::model::HORIZONTAL;
use crate::zelda3::model::NORTH;
use crate::zelda3::model::SNAKE;
use crate::zelda3::model::SOUTH;
use crate::zelda3::model::VERTICAL;
use crate::zelda3::model::WEST;

/// Returns the type of the sprite. This classification implies
/// if they can be killed, shuffled, etc. and is the most important
/// metadata about a sprite that isn't in the game data.
pub(crate) fn get_sprite_type(sprite_id: &SpriteId) -> SpriteType {
    match sprite_id {
        SpriteId::x0_RAVEN => SpriteType::Enemy,
        SpriteId::x1_VULTURE => SpriteType::Enemy,
        SpriteId::x2_STALFOS_HEAD => SpriteType::Object,
        SpriteId::x3_NONE => SpriteType::None,
        SpriteId::x4_PULL_SWITCH_NORMAL => SpriteType::Object,
        SpriteId::x5_PULL_SWITCH_NORMAL_UNUSED => SpriteType::Object,
        SpriteId::x6_PULL_SWITCH_TRAP => SpriteType::Object,
        SpriteId::x7_PULL_SWITCH_TRAP_UNUSED => SpriteType::Object,
        SpriteId::x8_OCTOROK => SpriteType::Enemy,
        SpriteId::x9_MOLDORM => SpriteType::Boss,
        SpriteId::xA_OCTOROK_FOUR_WAY => SpriteType::Enemy,
        SpriteId::xB_CUCCO => SpriteType::Npc,
        SpriteId::xC_OCTOROK_STONE => SpriteType::Object,
        SpriteId::xD_BUZZBLOB => SpriteType::Enemy,
        SpriteId::xE_SNAPDRAGON => SpriteType::Enemy,
        SpriteId::xF_OCTOBALLOON => SpriteType::Enemy,
        SpriteId::x10_OCTOBALLOON_HATCHLINGS => SpriteType::Enemy,
        SpriteId::x11_HINOX => SpriteType::Enemy,
        SpriteId::x12_MOBLIN => SpriteType::Enemy,
        SpriteId::x13_MINI_HELMASAUR => SpriteType::Enemy,
        SpriteId::x14_THIEVES_TOWN_GRATE => SpriteType::Object,
        SpriteId::x15_ANTIFAIRY => SpriteType::Hazard,
        SpriteId::x16_SAHASRAHLA => SpriteType::Npc,
        SpriteId::x17_HOARDER => SpriteType::Enemy,
        SpriteId::x18_MINI_MOLDORM => SpriteType::Enemy,
        SpriteId::x19_POE => SpriteType::Enemy,
        SpriteId::x1A_SMITHY => SpriteType::Npc,
        SpriteId::x1B_ARROW => SpriteType::Object,
        SpriteId::x1C_STATUE => SpriteType::Object,
        SpriteId::x1D_FLUTEQUEST => SpriteType::Object,
        SpriteId::x1E_CRYSTAL_SWITCH => SpriteType::Object,
        SpriteId::x1F_BUG_CATCHING_KID => SpriteType::Npc,
        SpriteId::x20_SLUGGULA => SpriteType::Enemy,
        SpriteId::x21_WATER_SWITCH => SpriteType::Object,
        SpriteId::x22_ROPA => SpriteType::Enemy,
        SpriteId::x23_RED_BARI => SpriteType::Enemy,
        SpriteId::x24_BLUE_BARI => SpriteType::Enemy,
        SpriteId::x25_TALKING_TREE => SpriteType::Npc,
        SpriteId::x26_HARDHAT_BEETLE => SpriteType::Enemy,
        SpriteId::x27_DEADROCK => SpriteType::Enemy,
        SpriteId::x28_HINT_PC_DW => SpriteType::Npc,
        SpriteId::x29_BLIND_HIDEOUT_ATTENDANT => SpriteType::Npc,
        SpriteId::x2A_SWEEPING_LADY => SpriteType::Npc,
        SpriteId::x2B_TENTMAN => SpriteType::Npc,
        SpriteId::x2C_LUMBERJACKS => SpriteType::Npc,
        SpriteId::x2D_NECKLESS_MAN => SpriteType::Object,
        SpriteId::x2E_FLUTE_KID => SpriteType::Npc,
        SpriteId::x2F_RACE_LADY => SpriteType::Npc,
        SpriteId::x30_RACE_GUY => SpriteType::Npc,
        SpriteId::x31_FORTUNE_TELLER => SpriteType::Npc,
        SpriteId::x32_ANGRY_BROTHERS => SpriteType::Npc,
        SpriteId::x33_RUPEE_PULL => SpriteType::Object,
        SpriteId::x34_SNITCH_YOUNG => SpriteType::Npc,
        SpriteId::x35_INNKEEPER => SpriteType::Npc,
        SpriteId::x36_WITCH => SpriteType::Npc,
        SpriteId::x37_WATERFALL => SpriteType::Object,
        SpriteId::x38_EYEGORE_STATUE => SpriteType::Object,
        SpriteId::x39_LOCKSMITH => SpriteType::Npc,
        SpriteId::x3A_MAGIC_BAT => SpriteType::Npc,
        SpriteId::x3B_BONK_ITEM => SpriteType::Object,
        SpriteId::x3C_VILLAGE_KID => SpriteType::Npc,
        SpriteId::x3D_SNITCH_OLD => SpriteType::Npc,
        SpriteId::x3E_HOARDER_ROCK => SpriteType::Enemy,
        SpriteId::x3F_TUTORIAL_SOLDIER => SpriteType::Npc,
        SpriteId::x40_LIGHTNING_LOCK => SpriteType::Object,
        SpriteId::x41_BLUE_GUARD => SpriteType::Enemy,
        SpriteId::x42_GREEN_GUARD => SpriteType::Enemy,
        SpriteId::x43_RED_SPEAR_GUARD => SpriteType::Enemy,
        SpriteId::x44_BLUE_ASSAULT_GUARD => SpriteType::Enemy,
        SpriteId::x45_BLACK_SPEAR_GUARD => SpriteType::Enemy,
        SpriteId::x46_BLUE_ARCHER => SpriteType::Enemy,
        SpriteId::x47_GREEN_GUARD_BUSH => SpriteType::Enemy,
        SpriteId::x48_RED_JAVELIN_GUARD => SpriteType::Enemy,
        SpriteId::x49_RED_GUARD_BUSH => SpriteType::Enemy,
        SpriteId::x4A_RED_BOMB_GUARD => SpriteType::Enemy,
        SpriteId::x4B_GREEN_KNIFE_GUARD => SpriteType::Enemy,
        SpriteId::x4C_GELDMAN => SpriteType::Enemy,
        SpriteId::x4D_TOPPO => SpriteType::Creature,
        SpriteId::x4E_POPO_1 => SpriteType::Enemy,
        SpriteId::x4F_POPO_2 => SpriteType::Enemy,
        SpriteId::x50_CANNON_BALL => SpriteType::Object,
        SpriteId::x51_ARMOS_STATUE => SpriteType::Enemy,
        SpriteId::x52_ZORA_KING => SpriteType::Npc,
        SpriteId::x53_ARMOS_KNIGHT => SpriteType::Boss,
        SpriteId::x54_LANMOLAS => SpriteType::Boss,
        SpriteId::x55_FIREBALL_ZORA => SpriteType::Enemy,
        SpriteId::x56_WALKING_ZORA => SpriteType::Enemy,
        SpriteId::x57_DESERT_STATUE => SpriteType::Object,
        SpriteId::x58_CRAB => SpriteType::Enemy,
        SpriteId::x59_LOST_WOODS_BIRD => SpriteType::Creature,
        SpriteId::x5A_LOST_WOODS_SQUIRREL => SpriteType::Creature,
        SpriteId::x5B_SPARK_CLOCKWISE => SpriteType::Hazard,
        SpriteId::x5C_SPARK_COUNTER_CLOCKWISE => SpriteType::Hazard,
        SpriteId::x5D_ROLLER_SOUTH => SpriteType::Hazard,
        SpriteId::x5E_ROLLER_NORTH => SpriteType::Hazard,
        SpriteId::x5F_ROLLER_EAST => SpriteType::Hazard,
        SpriteId::x60_ROLLER_WEST => SpriteType::Hazard,
        SpriteId::x61_BEAMOS => SpriteType::Hazard,
        SpriteId::x62_MASTERSWORD => SpriteType::Object,
        SpriteId::x63_DEVALANT_PIT => SpriteType::Hazard,
        SpriteId::x64_DEVALANT => SpriteType::Enemy,
        SpriteId::x65_ARCHERY_GUY => SpriteType::Npc,
        SpriteId::x66_MOVING_CANNON_EAST => SpriteType::Hazard,
        SpriteId::x67_MOVING_CANNON_WEST => SpriteType::Hazard,
        SpriteId::x68_MOVING_CANNON_SOUTH => SpriteType::Hazard,
        SpriteId::x69_MOVING_CANNON_NORTH => SpriteType::Hazard,
        SpriteId::x6A_BALL_N_CHAIN_GUARD => SpriteType::Enemy,
        SpriteId::x6B_CANNON_GUARD => SpriteType::Enemy,
        SpriteId::x6C_MIRROR_PORTAL => SpriteType::Object,
        SpriteId::x6D_RAT_CRICKET => SpriteType::Enemy,
        SpriteId::x6E_ROPE => SpriteType::Enemy,
        SpriteId::x6F_KEESE => SpriteType::Enemy,
        SpriteId::x70_HELMASAUR_KING_FIREBALL => SpriteType::Object,
        SpriteId::x71_LEEVER => SpriteType::Enemy,
        SpriteId::x72_FAIRY_POND_TRIGGER => SpriteType::Object,
        SpriteId::x73_UNCLE_PRIEST_MANTLE => SpriteType::Npc,
        SpriteId::x74_RUNNING_MAN => SpriteType::Npc,
        SpriteId::x75_BOTTLE_SALESMAN => SpriteType::Npc,
        SpriteId::x76_ZELDA => SpriteType::Npc,
        SpriteId::x77_ANTIFAIRY_2 => SpriteType::Enemy,
        SpriteId::x78_VILLAGE_ELDER => SpriteType::Npc,
        SpriteId::x79_BEE => SpriteType::Absorbable,
        SpriteId::x7A_AGAHNIM => SpriteType::Boss,
        SpriteId::x7B_AGAHNIM_ENERGY_BALL => SpriteType::Object,
        SpriteId::x7C_GREEN_STALFOS => SpriteType::Enemy,
        SpriteId::x7D_BIG_SPIKE => SpriteType::Hazard,
        SpriteId::x7E_FIREBAR_CLOCKWISE => SpriteType::Hazard,
        SpriteId::x7F_FIREBAR_COUNTER_CLOCKWISE => SpriteType::Hazard,
        SpriteId::x80_FIRESNAKE => SpriteType::Hazard,
        SpriteId::x81_WATER_TEKTITE => SpriteType::Enemy,
        SpriteId::x82_ANTIFAIRY_CIRCLE => SpriteType::Hazard,
        SpriteId::x83_GREEN_EYEGORE => SpriteType::Enemy,
        SpriteId::x84_RED_EYEGORE => SpriteType::Enemy,
        SpriteId::x85_YELLOW_STALFOS => SpriteType::Enemy,
        SpriteId::x86_KODONGO => SpriteType::Enemy,
        SpriteId::x87_KODONGO_FIRE => SpriteType::Object,
        SpriteId::x88_MOTHULA => SpriteType::Boss,
        SpriteId::x89_MOTHULA_BEAM => SpriteType::Object,
        SpriteId::x8A_SPIKE_BLOCK => SpriteType::Hazard,
        SpriteId::x8B_GIBDO => SpriteType::Enemy,
        SpriteId::x8C_ARRGHUS => SpriteType::Boss,
        SpriteId::x8D_ARRGHUS_SPAWN => SpriteType::Enemy,
        SpriteId::x8E_TERRORPIN => SpriteType::Enemy,
        SpriteId::x8F_BLOB => SpriteType::Enemy,
        SpriteId::x90_WALLMASTER => SpriteType::Overlord,
        SpriteId::x91_STALFOS_KNIGHT => SpriteType::Enemy,
        SpriteId::x92_KING_HELMASAUR => SpriteType::Boss,
        SpriteId::x93_BUMPER => SpriteType::Hazard,
        SpriteId::x94_PIROGUSU => SpriteType::Enemy,
        SpriteId::x95_EYE_LASER_EAST => SpriteType::Hazard,
        SpriteId::x96_EYE_LASER_WEST => SpriteType::Hazard,
        SpriteId::x97_EYE_LASER_SOUTH => SpriteType::Hazard,
        SpriteId::x98_EYE_LASER_NORTH => SpriteType::Hazard,
        SpriteId::x99_PENGATOR => SpriteType::Enemy,
        SpriteId::x9A_KYAMERON => SpriteType::Enemy,
        SpriteId::x9B_WIZZROBE => SpriteType::Enemy,
        SpriteId::x9C_BABASU_EAST => SpriteType::Hazard,
        SpriteId::x9D_BABUSU_SOUTH => SpriteType::Hazard,
        SpriteId::x9E_HAUNTED_GROVE_OSTRICH => SpriteType::Creature,
        SpriteId::x9F_HAUNTED_GROVE_RABBIT => SpriteType::Creature,
        SpriteId::xA0_HAUNTED_GROVE_BIRD => SpriteType::Creature,
        SpriteId::xA1_FREEZOR => SpriteType::Enemy,
        SpriteId::xA2_KHOLDSTARE => SpriteType::Boss,
        SpriteId::xA3_KHOLDSTARES_SHELL => SpriteType::Boss,
        SpriteId::xA4_FALLING_ICE => SpriteType::Hazard,
        SpriteId::xA5_BLUE_ZAZAK => SpriteType::Enemy,
        SpriteId::xA6_RED_ZAZAK => SpriteType::Enemy,
        SpriteId::xA7_STALFOS => SpriteType::Enemy,
        SpriteId::xA8_GREEN_ZIRRO => SpriteType::Enemy,
        SpriteId::xA9_BLUE_ZIRRO => SpriteType::Enemy,
        SpriteId::xAA_PIKIT_LIKE_LIKE => SpriteType::Enemy,
        SpriteId::xAB_CRYSTAL_MAIDEN => SpriteType::Npc,
        SpriteId::xAC_APPLE => SpriteType::Absorbable,
        SpriteId::xAD_LOST_OLD_MAN => SpriteType::Npc,
        SpriteId::xAE_DOWN_PIPE => SpriteType::Object,
        SpriteId::xAF_UP_PIPE => SpriteType::Object,
        SpriteId::xB0_RIGHT_PIPE => SpriteType::Object,
        SpriteId::xB1_LEFT_PIPE => SpriteType::Object,
        SpriteId::xB2_GOOD_BEE => SpriteType::Npc,
        SpriteId::xB3_PEDESTAL_PLAQUE => SpriteType::Object,
        SpriteId::xB4_PURPLE_CHEST => SpriteType::Object,
        SpriteId::xB5_BOMB_SALESMAN => SpriteType::Npc,
        SpriteId::xB6_KIKI => SpriteType::Npc,
        SpriteId::xB7_BLIND_MAIDEN => SpriteType::Npc,
        SpriteId::xB8_MIMIC => SpriteType::Enemy,
        SpriteId::xB9_BULLY_AND_FRIEND => SpriteType::Npc,
        SpriteId::xBA_WHIRLPOOL => SpriteType::Object,
        SpriteId::xBB_SALESMAN => SpriteType::Npc,
        SpriteId::xBC_DRUNK_IN_THE_INN => SpriteType::Npc,
        SpriteId::xBD_VITREOUS => SpriteType::Boss,
        SpriteId::xBE_VITREOUS_SMALL_EYEBALL => SpriteType::Boss,
        SpriteId::xBF_LIGHTNING => SpriteType::Boss,
        SpriteId::xC0_CATFISH => SpriteType::Npc,
        SpriteId::xC1_AGAHNIM_TELEPORTING => SpriteType::Npc,
        SpriteId::xC2_BOULDER => SpriteType::Object,
        SpriteId::xC3_GIBO => SpriteType::Enemy,
        SpriteId::xC4_THIEF => SpriteType::Enemy,
        SpriteId::xC5_MEDUSA => SpriteType::Hazard,
        SpriteId::xC6_MEDUSA_FOUR_WAY => SpriteType::Hazard,
        SpriteId::xC7_POKEY => SpriteType::Enemy,
        SpriteId::xC8_GREAT_FAIRY => SpriteType::Npc,
        SpriteId::xC9_TEKTITE => SpriteType::Enemy,
        SpriteId::xCA_CHAIN_CHOMP => SpriteType::Hazard,
        SpriteId::xCB_TRINEXX_ROCK => SpriteType::Boss,
        SpriteId::xCC_TRINEXX_FIRE => SpriteType::Boss,
        SpriteId::xCD_TRINEXX_ICE => SpriteType::Boss,
        SpriteId::xCE_BLIND => SpriteType::Boss,
        SpriteId::xCF_SWAMOLA => SpriteType::Enemy,
        SpriteId::xD0_LYNEL => SpriteType::Enemy,
        SpriteId::xD1_BUNNY_BEAM => SpriteType::Hazard,
        SpriteId::xD2_FLOPPING_FISH => SpriteType::Creature,
        // Set stal as hazards since they require activation and should not be
        // considered alive for the purpose of kill rooms. Setting this to hazard
        // should also flag the sprite as statis.
        SpriteId::xD3_STAL => SpriteType::Hazard,
        SpriteId::xD4_LANDMINE => SpriteType::Hazard,
        SpriteId::xD5_DIGGING_GAME_PROPRIETOR => SpriteType::Npc,
        SpriteId::xD6_GANON => SpriteType::Boss,
        SpriteId::xD7_GANON_INVINCIBLE => SpriteType::Npc,
        SpriteId::xD8_HEART => SpriteType::Absorbable,
        SpriteId::xD9_GREEN_RUPEE => SpriteType::Absorbable,
        SpriteId::xDA_BLUE_RUPEE => SpriteType::Absorbable,
        SpriteId::xDB_RED_RUPEE => SpriteType::Absorbable,
        SpriteId::xDC_BOMB_REFILL_1 => SpriteType::Absorbable,
        SpriteId::xDD_BOMB_REFILL_4 => SpriteType::Absorbable,
        SpriteId::xDE_BOMB_REFILL_8 => SpriteType::Absorbable,
        SpriteId::xDF_SMALL_MAGIC_REFILL => SpriteType::Absorbable,
        SpriteId::xE0_FULL_MAGIC_REFILL => SpriteType::Absorbable,
        SpriteId::xE1_ARROW_REFILL_5 => SpriteType::Absorbable,
        SpriteId::xE2_ARROW_REFILL_10 => SpriteType::Absorbable,
        SpriteId::xE3_FAIRY => SpriteType::Absorbable,
        SpriteId::xE4_SMALL_KEY => SpriteType::Object,
        SpriteId::xE5_BIG_KEY => SpriteType::Object,
        SpriteId::xE6_SHIELD => SpriteType::Object,
        SpriteId::xE7_MUSHROOM => SpriteType::Object,
        SpriteId::xE8_FAKE_MASTER_SWORD => SpriteType::Object,
        SpriteId::xE9_MAGIC_MERCHANT => SpriteType::Npc,
        SpriteId::xEA_HEART_CONTAINER => SpriteType::Object,
        SpriteId::xEB_HEART_PIECE => SpriteType::Object,
        SpriteId::xEC_THROWN_ITEM => SpriteType::Object,
        SpriteId::xED_SOMARIA_PLATFORM => SpriteType::Object,
        SpriteId::xEE_CASTLE_MANTLE => SpriteType::Object,
        SpriteId::xEF_SOMARIA_PLATFORM_UNUSED_1 => SpriteType::Object,
        SpriteId::xF0_SOMARIA_PLATFORM_UNUSED_2 => SpriteType::Object,
        SpriteId::xF1_SOMARIA_PLATFORM_UNUSED_3 => SpriteType::Object,
        SpriteId::xF2_MEDALLION_TABLET => SpriteType::Object,
        SpriteId::xF3_PERSONS_DOOR_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::xF4_FALLING_ROCKS_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::xF5_CANON_BALLS_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::xF6_UNKNOWN_F6_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::xF7_UNKNOWN_F7_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::xF8_UNKNOWN_F8_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::xF9_UNKNOWN_F9_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::xFA_BLOB_DROP_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::xFB_OVERWORLD_WALLMASTER_OW_OVERLORD_DROPS_IN_HOULIHAN => SpriteType::Overlord,
        SpriteId::xFC_FLOOR_DROP_SQUARE_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::xFD_FLOOR_DROP_NORTH_PATH_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::xFE_FLOOR_DROP_EAST_PATH_OW_OVERLORD => SpriteType::Overlord,
        SpriteId::x102_CANON_BALLS_EP_4_WALL_CANONBALLS => SpriteType::Overlord,
        SpriteId::x103_CANON_BALLS_EP_ENTRY => SpriteType::Overlord,
        SpriteId::x106_BOMB_DROP1_TRAP => SpriteType::Overlord,
        SpriteId::x104_ROPE_DROP_TRAP => SpriteType::Overlord,
        SpriteId::x105_STALFOS_HEAD_TRAP => SpriteType::Overlord,
        SpriteId::x107_MOVING_FLOOR => SpriteType::Overlord,
        SpriteId::x108_TRANSFORMER_BUNNY_BEAM => SpriteType::Overlord,
        SpriteId::x109_WALLMASTER_OVERLORD => SpriteType::Overlord,
        SpriteId::x10A_FLOOR_DROP_SQUARE => SpriteType::Overlord,
        SpriteId::x10B_FLOOR_DROP_NORTH => SpriteType::Overlord,
        SpriteId::x110_PIROGUSU_SPAWNER_RIGHT => SpriteType::Overlord,
        SpriteId::x111_PIROGUSU_SPAWNER_LEFT => SpriteType::Overlord,
        SpriteId::x112_PIROGUSU_SPAWNER_DOWN => SpriteType::Overlord,
        SpriteId::x113_PIROGUSU_SPAWNER_UP => SpriteType::Overlord,
        SpriteId::x114_FLYING_FLOOR_TILE_TRAP => SpriteType::Overlord,
        SpriteId::x115_WIZZROBE_SPAWNER => SpriteType::Overlord,
        SpriteId::x116_ZORO_SPAWNER => SpriteType::Overlord,
        SpriteId::x117_FOUR_SKULL_TRAP_IN_POD_UNDER_POT => SpriteType::Overlord,
        SpriteId::x118_STALFOS_APPEAR => SpriteType::Overlord,
        SpriteId::x119_ARMOS_KNIGHTS_TRIGGER => SpriteType::Overlord,
        SpriteId::x11A_BOMB_DROP2_TRAP => SpriteType::Overlord,
        SpriteId::x141_SOLDIER_ALERTER_BLUE => SpriteType::Overlord,
        SpriteId::x142_SOLDIER_ALERTER_GREEN => SpriteType::Overlord,
    }
}

/// True if the sprite can swim or has special water capabilities.
pub(crate) fn can_sprite_swim(sprite_id: &SpriteId) -> bool {
    matches!(
        sprite_id,
        SpriteId::x55_FIREBALL_ZORA
            | SpriteId::x81_WATER_TEKTITE
            | SpriteId::x94_PIROGUSU
            | SpriteId::x9A_KYAMERON
            | SpriteId::xCF_SWAMOLA
            | SpriteId::xD2_FLOPPING_FISH
    )
}

/// True if the sprite is flying -- although this mostly a signal
/// that the sprite has very simple collisions.
pub(crate) fn can_sprite_fly(sprite_id: &SpriteId) -> bool {
    matches!(
        sprite_id,
        SpriteId::x0_RAVEN
            | SpriteId::x1_VULTURE
            | SpriteId::xF_OCTOBALLOON
            | SpriteId::x19_POE
            | SpriteId::x23_RED_BARI
            | SpriteId::x24_BLUE_BARI
            | SpriteId::x6F_KEESE
            | SpriteId::x7C_GREEN_STALFOS
            | SpriteId::xA8_GREEN_ZIRRO
            | SpriteId::xA9_BLUE_ZIRRO
            | SpriteId::x85_YELLOW_STALFOS
            | SpriteId::x2_STALFOS_HEAD
            | SpriteId::x4C_GELDMAN
    )
}

/// True if the type implies that it can be shuffled automatically.
fn can_shuffle_type(sprite_id: &SpriteId) -> bool {
    matches!(
        get_sprite_type(sprite_id),
        SpriteType::Creature | SpriteType::Enemy | SpriteType::Hazard | SpriteType::Absorbable
    )
}

/// This is used to restrict shuffling in the overworld. If there isn't
/// a specific sprite lists, the implied shuffling rules of the type apply.
pub(crate) fn can_shuffle_in_overworld(sprite_id: &SpriteId) -> bool {
    match sprite_id {
        // Toppo spawning on a cave/entrance will allow for the message
        // he says to occur at the same time as teleporting. This crashes
        // the game. Toppo also ends up toppled if there isn't grass underneath.
        // That isn't detectable at this time.
        SpriteId::x4D_TOPPO => false,
        // Not applicable to overworld.
        SpriteId::x90_WALLMASTER => false,
        // They don't interact with overworld collisions super well.
        SpriteId::x94_PIROGUSU => false,
        // TODO: look at patching away the smoke behavior and instead
        // allowing bunny beam in the overworld.
        SpriteId::xD1_BUNNY_BEAM => false,
        _ => can_shuffle_type(sprite_id),
    }
}

/// This is used to restrict shuffling in the underworld. If there isn't
/// a specific sprite lists, the implied shuffling rules of the type apply.
pub(crate) fn can_shuffle_in_underworld(sprite_id: &SpriteId) -> bool {
    match sprite_id {
        SpriteId::x82_ANTIFAIRY_CIRCLE => false,
        // Needs investigation
        SpriteId::x8D_ARRGHUS_SPAWN => false,
        // Only should spawn from overlord.
        SpriteId::x90_WALLMASTER => false,
        // Only should spawn from overlord.
        SpriteId::x94_PIROGUSU => false,
        // Needs investigation
        SpriteId::xA1_FREEZOR => false,
        // Graphics loaded in memory only in Overworld.
        SpriteId::xD4_LANDMINE => false,
        _ => can_shuffle_type(sprite_id),
    }
}

/// This function indicates which sprites are eligible to hold keys
/// in the underworld. Typical reasons something is not eligible are
/// 1. They move off screen (mostly flying)
/// 2. They are destroyed without a kill condition (RED BARI, HOARDER)
/// 3. They inexplicably crash the game (MOBLIN supposedly does this)
pub(crate) fn can_sprite_hold_key(sprite_id: &SpriteId) -> bool {
    match sprite_id {
        // Has a reputation to crash the game.
        SpriteId::x12_MOBLIN => false,
        // Loses the key.
        SpriteId::x17_HOARDER => false,
        // Loses the key.
        SpriteId::x23_RED_BARI => false,
        // Loses the key.
        SpriteId::x3E_HOARDER_ROCK => false,
        // Exits the screen.
        SpriteId::x4C_GELDMAN => false,
        // Exits the screen.
        SpriteId::x6F_KEESE => false,
        // Exits the screen.
        SpriteId::x85_YELLOW_STALFOS => false,
        // Needs investigation.
        SpriteId::x9C_BABASU_EAST => false,
        // Needs investigation.
        SpriteId::x9D_BABUSU_SOUTH => false,
        // Exits the screen.
        SpriteId::xA8_GREEN_ZIRRO => false,
        // Exits the screen.
        SpriteId::xA9_BLUE_ZIRRO => false,
        _ => matches!(
            get_sprite_type(sprite_id),
            SpriteType::Enemy | SpriteType::Boss
        ),
    }
}

pub(crate) fn sprite_movement(sprite_id: &SpriteId) -> Option<u8> {
    match sprite_id {
        SpriteId::x15_ANTIFAIRY => Some(DIAGONAL),
        SpriteId::x5B_SPARK_CLOCKWISE => Some(SNAKE | DIAGONAL),
        SpriteId::x5C_SPARK_COUNTER_CLOCKWISE => Some(SNAKE | DIAGONAL),
        SpriteId::x5D_ROLLER_SOUTH => Some(SOUTH),
        SpriteId::x5E_ROLLER_NORTH => Some(NORTH),
        SpriteId::x5F_ROLLER_EAST => Some(EAST),
        SpriteId::x60_ROLLER_WEST => Some(WEST),
        SpriteId::x61_BEAMOS => Some(FIXED),
        SpriteId::x63_DEVALANT_PIT => Some(FIXED),
        SpriteId::x66_MOVING_CANNON_EAST => Some(EAST),
        SpriteId::x67_MOVING_CANNON_WEST => Some(WEST),
        SpriteId::x68_MOVING_CANNON_SOUTH => Some(SOUTH),
        SpriteId::x69_MOVING_CANNON_NORTH => Some(NORTH),
        SpriteId::x77_ANTIFAIRY_2 => Some(DIAGONAL),
        SpriteId::x7D_BIG_SPIKE => Some(HORIZONTAL | VERTICAL),
        SpriteId::x7E_FIREBAR_CLOCKWISE => Some(FIXED),
        SpriteId::x7F_FIREBAR_COUNTER_CLOCKWISE => Some(FIXED),
        SpriteId::x80_FIRESNAKE => Some(SNAKE | DIAGONAL),
        SpriteId::x82_ANTIFAIRY_CIRCLE => Some(DIAGONAL),
        SpriteId::x8A_SPIKE_BLOCK => Some(HORIZONTAL),
        SpriteId::x93_BUMPER => Some(FIXED),
        SpriteId::x95_EYE_LASER_EAST => Some(EAST),
        SpriteId::x96_EYE_LASER_WEST => Some(WEST),
        SpriteId::x97_EYE_LASER_SOUTH => Some(SOUTH),
        SpriteId::x98_EYE_LASER_NORTH => Some(NORTH),
        SpriteId::x9C_BABASU_EAST => Some(SOUTH),
        SpriteId::x9D_BABUSU_SOUTH => Some(EAST),
        SpriteId::xA4_FALLING_ICE => Some(DIAGONAL),
        SpriteId::xC5_MEDUSA => Some(FIXED),
        SpriteId::xC6_MEDUSA_FOUR_WAY => Some(FIXED),
        SpriteId::xCA_CHAIN_CHOMP => Some(FIXED),
        SpriteId::xD1_BUNNY_BEAM => Some(DIAGONAL | FIXED),
        SpriteId::xD3_STAL => Some(DIAGONAL),
        _ => None,
    }
}

pub(crate) fn get_sprite_requirements(sprite_id: &SpriteId) -> Vec<[SpriteSheetId; 4]> {
    match sprite_id {
        SpriteId::x0_RAVEN => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x19_SWAMOLA_CROW,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x11_MISC_FAKE_SWORD,
            ],
        ],
        SpriteId::x1_VULTURE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x12_DESERT_1,
            SpriteSheetId::None,
        ]],
        SpriteId::x2_STALFOS_HEAD => vec![[
            SpriteSheetId::x1F_STALFOS_BARI,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        // This should never match anything. It will break the game, seriously!
        SpriteId::x3_NONE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x4_PULL_SWITCH_NORMAL => vec![
            [
                SpriteSheetId::x46_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x55_AGAHNIM,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x5_PULL_SWITCH_NORMAL_UNUSED => vec![
            [
                SpriteSheetId::x46_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x55_AGAHNIM,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x6_PULL_SWITCH_TRAP => vec![
            [
                SpriteSheetId::x46_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x55_AGAHNIM,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x7_PULL_SWITCH_TRAP_UNUSED => vec![
            [
                SpriteSheetId::x46_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x55_AGAHNIM,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x8_OCTOROK => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::xC_OCTOROK_ZORA,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x18_OCTOROCKS,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x9_MOLDORM => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x30_MOLDORM_BOSS,
            SpriteSheetId::None,
        ]],
        SpriteId::xA_OCTOROK_FOUR_WAY => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::xC_OCTOROK_ZORA,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x18_OCTOROCKS,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::xB_CUCCO => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::xE_POE_THIEF_LW,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x15_POE_THIEF_DW,
            ],
        ],
        SpriteId::xC_OCTOROK_STONE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x18_OCTOROCKS,
            SpriteSheetId::None,
        ]],
        SpriteId::xD_BUZZBLOB => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x11_MISC_FAKE_SWORD,
        ]],
        SpriteId::xE_SNAPDRAGON => vec![[
            SpriteSheetId::x16_HINOX_SNAPDRAGON,
            SpriteSheetId::None,
            SpriteSheetId::x17_MOBLIN,
            SpriteSheetId::None,
        ]],
        SpriteId::xF_OCTOBALLOON => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::xC_OCTOROK_ZORA,
            SpriteSheetId::None,
        ]],
        SpriteId::x10_OCTOBALLOON_HATCHLINGS => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::xC_OCTOROK_ZORA,
            SpriteSheetId::None,
        ]],
        SpriteId::x11_HINOX => vec![[
            SpriteSheetId::x16_HINOX_SNAPDRAGON,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x12_MOBLIN => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x17_MOBLIN,
            SpriteSheetId::None,
        ]],
        SpriteId::x13_MINI_HELMASAUR => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x1E_MINI_MONSTERS,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x15_ANTIFAIRY => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
        ]],
        SpriteId::x16_SAHASRAHLA => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4C_SAHASRAHLA_WITCH,
            SpriteSheetId::None,
        ]],
        SpriteId::x17_HOARDER => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x11_MISC_FAKE_SWORD,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x10_MISC_ROCKS,
            ],
        ],
        SpriteId::x18_MINI_MOLDORM => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x1E_MINI_MONSTERS,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x19_POE => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::xE_POE_THIEF_LW,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x15_POE_THIEF_DW,
            ],
            // Homebrew (floating hoarder 1)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x10_MISC_ROCKS,
            ],
            // Homebrew (floating hoarder 2)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x11_MISC_FAKE_SWORD,
            ],
            // Homebrew (floating zirro)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x1B_MISCELLANEOUS_DW_1,
            ],
            // Homebrew (floating skull)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x20_STALFOS_KNIGHT_VERMIN,
            ],
            // Homebrew (floating orb)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x25_WIZZROBE_SLUGGULA,
            ],
            // Homebrew (floating orb)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x26_FROSTY_FRIENDS,
            ],
            // Homebrew (floating orb)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x36_MASTER_SWORD,
            ],
            // Homebrew (floating orb)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x39_ARRGHUS_BOSS,
            ],
            // Homebrew (floating eye)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x3D_VITREOUS_BOSS,
            ],
            // Homebrew (floating eye)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x41_BIG_BAD_GUY,
            ],
            // Homebrew (floating rabbit)
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
            ],
        ],
        SpriteId::x1A_SMITHY => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4A_KAKARIKO,
            SpriteSheetId::None,
        ]],
        SpriteId::x1C_STATUE => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x52_PULLSWITCH_CRYSTAL_SPIKE_BOUNCER,
            ],
        ],
        SpriteId::x1D_FLUTEQUEST => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
            SpriteSheetId::None,
        ]],
        SpriteId::x1E_CRYSTAL_SWITCH => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x52_PULLSWITCH_CRYSTAL_SPIKE_BOUNCER,
            ],
        ],
        SpriteId::x1F_BUG_CATCHING_KID => vec![[
            SpriteSheetId::x51_UNCLE_PRIEST_SICK_BOY,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x20_SLUGGULA => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x25_WIZZROBE_SLUGGULA,
            SpriteSheetId::None,
        ]],
        SpriteId::x21_WATER_SWITCH => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
        ]],
        SpriteId::x22_ROPA => vec![[
            SpriteSheetId::x16_HINOX_SNAPDRAGON,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x23_RED_BARI => vec![[
            SpriteSheetId::x1F_STALFOS_BARI,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x24_BLUE_BARI => vec![[
            SpriteSheetId::x1F_STALFOS_BARI,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x25_TALKING_TREE => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x1B_MISCELLANEOUS_DW_1,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x19_SWAMOLA_CROW,
            ],
        ],
        SpriteId::x26_HARDHAT_BEETLE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x1E_MINI_MONSTERS,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x27_DEADROCK => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::x28_HINT_PC_DW => vec![[
            SpriteSheetId::x4B_ARCHERY,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x29_BLIND_HIDEOUT_ATTENDANT => vec![[
            SpriteSheetId::x4F_OLD_MAN_RUNNER,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x2A_SWEEPING_LADY => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4A_KAKARIKO,
            SpriteSheetId::None,
        ]],
        SpriteId::x2B_TENTMAN => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x37_MASTER_SWORD,
            SpriteSheetId::None,
        ]],
        SpriteId::x2C_LUMBERJACKS => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4A_KAKARIKO,
            SpriteSheetId::None,
        ]],
        SpriteId::x2E_FLUTE_KID => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x4C_SAHASRAHLA_WITCH,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x2F_RACE_LADY => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x50_CUCCO_FOR_NPCS,
        ]],
        SpriteId::x30_RACE_GUY => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x50_CUCCO_FOR_NPCS,
        ]],
        SpriteId::x31_FORTUNE_TELLER => vec![[
            SpriteSheetId::x4B_ARCHERY,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x32_ANGRY_BROTHERS => vec![[
            SpriteSheetId::x4F_OLD_MAN_RUNNER,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x34_SNITCH_YOUNG => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x50_CUCCO_FOR_NPCS,
        ]],
        SpriteId::x36_WITCH => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4C_SAHASRAHLA_WITCH,
            SpriteSheetId::None,
        ]],
        SpriteId::x39_LOCKSMITH => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x12_DESERT_1,
            SpriteSheetId::x11_MISC_FAKE_SWORD,
        ]],
        SpriteId::x3A_MAGIC_BAT => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x1D_ARMOS_BOSS_LOCK_BAT,
        ]],
        SpriteId::x3B_BONK_ITEM => vec![[
            SpriteSheetId::xF_DASH_HOARDER,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x3C_VILLAGE_KID => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4A_KAKARIKO,
            SpriteSheetId::None,
        ]],
        SpriteId::x3D_SNITCH_OLD => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x50_CUCCO_FOR_NPCS,
        ]],
        SpriteId::x3E_HOARDER_ROCK => vec![
            [
                SpriteSheetId::xF_DASH_HOARDER,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x11_MISC_FAKE_SWORD,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x10_MISC_ROCKS,
            ],
        ],
        SpriteId::x3F_TUTORIAL_SOLDIER => vec![[
            SpriteSheetId::x48_SOLDIER,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x40_LIGHTNING_LOCK => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x1C_PESTS,
            SpriteSheetId::x1D_ARMOS_BOSS_LOCK_BAT,
        ]],
        SpriteId::x41_BLUE_GUARD => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::x49_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::xD_SOLDIERS_DW,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x42_GREEN_GUARD => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::x49_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::xD_SOLDIERS_DW,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x43_RED_SPEAR_GUARD => vec![
            [
                SpriteSheetId::x46_SOLDIERS,
                SpriteSheetId::x49_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                // Homebrew: Borrow the top half from lightworld.
                SpriteSheetId::x46_SOLDIERS,
                SpriteSheetId::xD_SOLDIERS_DW,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x44_BLUE_ASSAULT_GUARD => vec![
            [
                SpriteSheetId::x46_SOLDIERS,
                SpriteSheetId::x49_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x46_SOLDIERS,
                SpriteSheetId::xD_SOLDIERS_DW,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x45_BLACK_SPEAR_GUARD => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::x49_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::xD_SOLDIERS_DW,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x46_BLUE_ARCHER => vec![
            [
                SpriteSheetId::x48_SOLDIER,
                SpriteSheetId::x49_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x48_SOLDIER,
                SpriteSheetId::xD_SOLDIERS_DW,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x47_GREEN_GUARD_BUSH => vec![
            [
                SpriteSheetId::x48_SOLDIER,
                SpriteSheetId::x49_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x48_SOLDIER,
                SpriteSheetId::xD_SOLDIERS_DW,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x48_RED_JAVELIN_GUARD => vec![
            [
                SpriteSheetId::x46_SOLDIERS,
                SpriteSheetId::x49_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x48_SOLDIER,
                SpriteSheetId::xD_SOLDIERS_DW,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x49_RED_GUARD_BUSH => vec![
            [
                SpriteSheetId::x48_SOLDIER,
                SpriteSheetId::x49_SOLDIERS,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x48_SOLDIER,
                SpriteSheetId::xD_SOLDIERS_DW,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x4A_RED_BOMB_GUARD => vec![[
            SpriteSheetId::x46_SOLDIERS,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x4B_GREEN_KNIFE_GUARD => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::x49_SOLDIERS,
                SpriteSheetId::x13_SOLDIER_RECRUITS,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::xD_SOLDIERS_DW,
                SpriteSheetId::x13_SOLDIER_RECRUITS,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x4C_GELDMAN => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x12_DESERT_1,
            SpriteSheetId::None,
        ]],
        SpriteId::x4D_TOPPO => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x11_MISC_FAKE_SWORD,
        ]],
        SpriteId::x4E_POPO_1 => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2C_BEAM_ME_UP_MR_POPO,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x4F_POPO_2 => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2C_BEAM_ME_UP_MR_POPO,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x50_CANNON_BALL => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2E_EYEGORE,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x51_ARMOS_STATUE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::x52_ZORA_KING => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x44_ZORAS_DOMAIN,
        ]],
        SpriteId::x53_ARMOS_KNIGHT => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x1D_ARMOS_BOSS_LOCK_BAT,
        ]],
        SpriteId::x54_LANMOLAS => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x31_LANMOLAS_BOSS,
        ]],
        SpriteId::x55_FIREBALL_ZORA => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x18_OCTOROCKS,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::xC_OCTOROK_ZORA,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x56_WALKING_ZORA => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x44_ZORAS_DOMAIN,
        ]],
        SpriteId::x57_DESERT_STATUE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x12_DESERT_1,
            SpriteSheetId::None,
        ]],
        SpriteId::x58_CRAB => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::xC_OCTOROK_ZORA,
            SpriteSheetId::None,
        ]],
        SpriteId::x59_LOST_WOODS_BIRD => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x36_MASTER_SWORD,
        ]],
        SpriteId::x5A_LOST_WOODS_SQUIRREL => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x36_MASTER_SWORD,
        ]],
        SpriteId::x5B_SPARK_CLOCKWISE => vec![[
            SpriteSheetId::x1F_STALFOS_BARI,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x5C_SPARK_COUNTER_CLOCKWISE => vec![[
            SpriteSheetId::x1F_STALFOS_BARI,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x5D_ROLLER_SOUTH => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x27_TURTLE_ROCK,
            SpriteSheetId::None,
        ]],
        SpriteId::x5E_ROLLER_NORTH => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x27_TURTLE_ROCK,
            SpriteSheetId::None,
        ]],
        SpriteId::x5F_ROLLER_EAST => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x27_TURTLE_ROCK,
            SpriteSheetId::None,
        ]],
        SpriteId::x60_ROLLER_WEST => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x27_TURTLE_ROCK,
            SpriteSheetId::None,
        ]],
        SpriteId::x61_BEAMOS => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2C_BEAM_ME_UP_MR_POPO,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x62_MASTERSWORD => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x37_MASTER_SWORD,
            SpriteSheetId::x36_MASTER_SWORD,
        ]],
        SpriteId::x63_DEVALANT_PIT => vec![[
            SpriteSheetId::x2F_CANON_SANDCRAB,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x64_DEVALANT => vec![[
            SpriteSheetId::x2F_CANON_SANDCRAB,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x65_ARCHERY_GUY => vec![[
            SpriteSheetId::x4B_ARCHERY,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x66_MOVING_CANNON_EAST => vec![[
            SpriteSheetId::x2F_CANON_SANDCRAB,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x67_MOVING_CANNON_WEST => vec![[
            SpriteSheetId::x2F_CANON_SANDCRAB,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x68_MOVING_CANNON_SOUTH => vec![[
            SpriteSheetId::x2F_CANON_SANDCRAB,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x69_MOVING_CANNON_NORTH => vec![[
            SpriteSheetId::x2F_CANON_SANDCRAB,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x6A_BALL_N_CHAIN_GUARD => vec![[
            SpriteSheetId::x46_SOLDIERS,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x6B_CANNON_GUARD => vec![[
            SpriteSheetId::x46_SOLDIERS,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x6D_RAT_CRICKET => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x1C_PESTS,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x24_PESTS_DW,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x6E_ROPE => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x1C_PESTS,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x24_PESTS_DW,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x6F_KEESE => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x1C_PESTS,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x24_PESTS_DW,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x70_HELMASAUR_KING_FIREBALL => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3A_HELMASAUR_KING_BOSS,
            SpriteSheetId::x3E_HELMASAUR_KING_BOSS,
        ]],
        SpriteId::x71_LEEVER => vec![[
            SpriteSheetId::x2F_CANON_SANDCRAB,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x72_FAIRY_POND_TRIGGER => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x36_MASTER_SWORD,
        ]],
        SpriteId::x73_UNCLE_PRIEST_MANTLE => vec![
            [
                SpriteSheetId::x47_PRIEST,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x51_UNCLE_PRIEST_SICK_BOY,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x74_RUNNING_MAN => vec![[
            SpriteSheetId::x4F_OLD_MAN_RUNNER,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x75_BOTTLE_SALESMAN => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4A_KAKARIKO,
            SpriteSheetId::None,
        ]],
        SpriteId::x77_ANTIFAIRY_2 => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
        ]],
        SpriteId::x78_VILLAGE_ELDER => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4A_KAKARIKO,
            SpriteSheetId::None,
        ]],
        SpriteId::x7A_AGAHNIM => vec![[
            SpriteSheetId::x55_AGAHNIM,
            SpriteSheetId::x1A_AGAHNIM,
            SpriteSheetId::x42_AGAHNIM,
            SpriteSheetId::x43_AGAHNIM,
        ]],
        SpriteId::x7B_AGAHNIM_ENERGY_BALL => vec![[
            SpriteSheetId::x55_AGAHNIM,
            SpriteSheetId::x1A_AGAHNIM,
            SpriteSheetId::x42_AGAHNIM,
            SpriteSheetId::x43_AGAHNIM,
        ]],
        SpriteId::x7C_GREEN_STALFOS => vec![[
            SpriteSheetId::x1F_STALFOS_BARI,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x7D_BIG_SPIKE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x52_PULLSWITCH_CRYSTAL_SPIKE_BOUNCER,
        ]],
        SpriteId::x7E_FIREBAR_CLOCKWISE => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x2A_HAZARDS,
                SpriteSheetId::None,
            ],
            // Homebrew: chain chomp bar.
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x27_TURTLE_ROCK,
                SpriteSheetId::None,
            ],
            // Homebrew: eyeball bar
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x3C_KHOLDSTARE_BOSS,
                SpriteSheetId::None,
            ],
            // Homebrew: moldorm bar
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x30_MOLDORM_BOSS,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x7F_FIREBAR_COUNTER_CLOCKWISE => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x2A_HAZARDS,
                SpriteSheetId::None,
            ],
            // Homebrew: chain chomp bar.
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x27_TURTLE_ROCK,
                SpriteSheetId::None,
            ],
            // Homebrew: eyeball bar
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x3C_KHOLDSTARE_BOSS,
                SpriteSheetId::None,
            ],
            // Homebrew: moldorm bar
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x30_MOLDORM_BOSS,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x80_FIRESNAKE => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x2A_HAZARDS,
                SpriteSheetId::None,
            ],
            // Homebrew: chain chomp snake.
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x27_TURTLE_ROCK,
                SpriteSheetId::None,
            ],
            // Homebrew: eyeball snake
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x3C_KHOLDSTARE_BOSS,
                SpriteSheetId::None,
            ],
            // Homebrew: moldorm snake
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x30_MOLDORM_BOSS,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x81_WATER_TEKTITE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x22_WATER_TEKTITES,
            SpriteSheetId::None,
        ]],
        SpriteId::x82_ANTIFAIRY_CIRCLE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
        ]],
        SpriteId::x83_GREEN_EYEGORE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x2E_EYEGORE,
            SpriteSheetId::None,
        ]],
        SpriteId::x84_RED_EYEGORE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2C_BEAM_ME_UP_MR_POPO,
            SpriteSheetId::x2E_EYEGORE,
            SpriteSheetId::None,
        ]],
        SpriteId::x85_YELLOW_STALFOS => vec![[
            SpriteSheetId::x1F_STALFOS_BARI,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x86_KODONGO => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2A_HAZARDS,
            SpriteSheetId::x2A_HAZARDS,
            SpriteSheetId::None,
        ]],
        SpriteId::x87_KODONGO_FIRE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2A_HAZARDS,
            SpriteSheetId::x2A_HAZARDS,
            SpriteSheetId::None,
        ]],
        SpriteId::x88_MOTHULA => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x38_MOTHULA_BOSS,
            SpriteSheetId::None,
        ]],
        SpriteId::x89_MOTHULA_BEAM => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x38_MOTHULA_BOSS,
            SpriteSheetId::None,
        ]],
        SpriteId::x8A_SPIKE_BLOCK => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x52_PULLSWITCH_CRYSTAL_SPIKE_BOUNCER,
            ],
        ],
        SpriteId::x8B_GIBDO => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x23_WALLMASTER_GIBDO,
            SpriteSheetId::None,
        ]],
        SpriteId::x8C_ARRGHUS => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x39_ARRGHUS_BOSS,
            SpriteSheetId::None,
        ]],
        SpriteId::x8D_ARRGHUS_SPAWN => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x39_ARRGHUS_BOSS,
            SpriteSheetId::None,
        ]],
        SpriteId::x8E_TERRORPIN => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2A_HAZARDS,
            SpriteSheetId::x2A_HAZARDS,
            SpriteSheetId::None,
        ]],
        SpriteId::x8F_BLOB => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x20_STALFOS_KNIGHT_VERMIN,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x90_WALLMASTER => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x23_WALLMASTER_GIBDO,
            SpriteSheetId::None,
        ]],
        SpriteId::x91_STALFOS_KNIGHT => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x20_STALFOS_KNIGHT_VERMIN,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x92_KING_HELMASAUR => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3A_HELMASAUR_KING_BOSS,
            SpriteSheetId::x3E_HELMASAUR_KING_BOSS,
        ]],
        SpriteId::x93_BUMPER => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x52_PULLSWITCH_CRYSTAL_SPIKE_BOUNCER,
        ]],
        SpriteId::x94_PIROGUSU => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x95_EYE_LASER_EAST => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x52_PULLSWITCH_CRYSTAL_SPIKE_BOUNCER,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
            ],
        ],
        SpriteId::x96_EYE_LASER_WEST => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x52_PULLSWITCH_CRYSTAL_SPIKE_BOUNCER,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
            ],
        ],
        SpriteId::x97_EYE_LASER_SOUTH => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x52_PULLSWITCH_CRYSTAL_SPIKE_BOUNCER,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
            ],
        ],
        SpriteId::x98_EYE_LASER_NORTH => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x52_PULLSWITCH_CRYSTAL_SPIKE_BOUNCER,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x53_STATUE_CRYSTAL_SPIKE_BOUNCER,
            ],
        ],
        SpriteId::x99_PENGATOR => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x26_FROSTY_FRIENDS,
            SpriteSheetId::None,
        ]],
        SpriteId::x9A_KYAMERON => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x22_WATER_TEKTITES,
            SpriteSheetId::None,
        ]],
        SpriteId::x9B_WIZZROBE => vec![
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x25_WIZZROBE_SLUGGULA,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::x29_WIZZROBE,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::x9C_BABASU_EAST => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x20_STALFOS_KNIGHT_VERMIN,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x9D_BABUSU_SOUTH => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x20_STALFOS_KNIGHT_VERMIN,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::x9E_HAUNTED_GROVE_OSTRICH => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
            SpriteSheetId::None,
        ]],
        SpriteId::x9F_HAUNTED_GROVE_RABBIT => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
            SpriteSheetId::None,
        ]],
        SpriteId::xA0_HAUNTED_GROVE_BIRD => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
            SpriteSheetId::None,
        ]],
        SpriteId::xA1_FREEZOR => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x26_FROSTY_FRIENDS,
            SpriteSheetId::None,
        ]],
        SpriteId::xA2_KHOLDSTARE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3C_KHOLDSTARE_BOSS,
            SpriteSheetId::None,
        ]],
        SpriteId::xA3_KHOLDSTARES_SHELL => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3C_KHOLDSTARE_BOSS,
            SpriteSheetId::None,
        ]],
        SpriteId::xA4_FALLING_ICE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3C_KHOLDSTARE_BOSS,
            SpriteSheetId::None,
        ]],
        SpriteId::xA5_BLUE_ZAZAK => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x28_ZAZAK,
            SpriteSheetId::None,
        ]],
        SpriteId::xA6_RED_ZAZAK => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x28_ZAZAK,
            SpriteSheetId::None,
        ]],
        SpriteId::xA7_STALFOS => vec![[
            SpriteSheetId::x1F_STALFOS_BARI,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::xA8_GREEN_ZIRRO => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x1B_MISCELLANEOUS_DW_1,
        ]],
        SpriteId::xA9_BLUE_ZIRRO => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x1B_MISCELLANEOUS_DW_1,
        ]],
        SpriteId::xAA_PIKIT_LIKE_LIKE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x1B_MISCELLANEOUS_DW_1,
        ]],
        SpriteId::xAB_CRYSTAL_MAIDEN => vec![[
            SpriteSheetId::x55_AGAHNIM,
            SpriteSheetId::x4D_OLD_MAN_MAIDEN,
            SpriteSheetId::x42_AGAHNIM,
            SpriteSheetId::x43_AGAHNIM,
        ]],
        SpriteId::xAD_LOST_OLD_MAN => vec![[
            SpriteSheetId::x4F_OLD_MAN_RUNNER,
            SpriteSheetId::x4D_OLD_MAN_MAIDEN,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::xB3_PEDESTAL_PLAQUE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x12_DESERT_1,
            SpriteSheetId::None,
        ]],
        SpriteId::xB4_PURPLE_CHEST => vec![[
            SpriteSheetId::x15_POE_THIEF_DW,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x15_POE_THIEF_DW,
        ]],
        SpriteId::xB5_BOMB_SALESMAN => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x4D_OLD_MAN_MAIDEN,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::xB6_KIKI => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x19_SWAMOLA_CROW,
        ]],
        SpriteId::xB7_BLIND_MAIDEN => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::xB8_MIMIC => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2C_BEAM_ME_UP_MR_POPO,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::xB9_BULLY_AND_FRIEND => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x14_FRIENDLY_LYNEL,
        ]],
        SpriteId::xBA_WHIRLPOOL => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::xC_OCTOROK_ZORA,
            SpriteSheetId::None,
        ]],
        SpriteId::xBB_SALESMAN => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x4A_KAKARIKO,
            SpriteSheetId::None,
        ]],
        SpriteId::xBD_VITREOUS => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3D_VITREOUS_BOSS,
        ]],
        SpriteId::xBE_VITREOUS_SMALL_EYEBALL => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3D_VITREOUS_BOSS,
        ]],
        SpriteId::xBF_LIGHTNING => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3D_VITREOUS_BOSS,
        ]],
        SpriteId::xC0_CATFISH => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x18_OCTOROCKS,
            SpriteSheetId::None,
        ]],
        SpriteId::xC1_AGAHNIM_TELEPORTING => vec![[
            SpriteSheetId::x55_AGAHNIM,
            SpriteSheetId::x1A_AGAHNIM,
            SpriteSheetId::x42_AGAHNIM,
            SpriteSheetId::x43_AGAHNIM,
        ]],
        SpriteId::xC2_BOULDER => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::xC3_GIBO => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x28_ZAZAK,
            SpriteSheetId::None,
        ]],
        SpriteId::xC4_THIEF => vec![
            [
                SpriteSheetId::xE_POE_THIEF_LW,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
            [
                SpriteSheetId::x15_POE_THIEF_DW,
                SpriteSheetId::None,
                SpriteSheetId::None,
                SpriteSheetId::None,
            ],
        ],
        SpriteId::xC7_POKEY => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x27_TURTLE_ROCK,
            SpriteSheetId::None,
        ]],
        SpriteId::xC8_GREAT_FAIRY => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x39_ARRGHUS_BOSS,
            SpriteSheetId::None,
        ]],
        SpriteId::xC9_TEKTITE => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::xCA_CHAIN_CHOMP => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x27_TURTLE_ROCK,
            SpriteSheetId::None,
        ]],
        SpriteId::xCB_TRINEXX_ROCK => vec![[
            SpriteSheetId::x40_TRINEXX_BOSS,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3F_TRINEXX_BOSS,
        ]],
        SpriteId::xCC_TRINEXX_FIRE => vec![[
            SpriteSheetId::x40_TRINEXX_BOSS,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3F_TRINEXX_BOSS,
        ]],
        SpriteId::xCD_TRINEXX_ICE => vec![[
            SpriteSheetId::x40_TRINEXX_BOSS,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x3F_TRINEXX_BOSS,
        ]],
        SpriteId::xCE_BLIND => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2C_BEAM_ME_UP_MR_POPO, // Frickin laser beams.
            SpriteSheetId::x3B_BLIND_BOSS,
            SpriteSheetId::None,
        ]],
        SpriteId::xCF_SWAMOLA => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x19_SWAMOLA_CROW,
        ]],
        SpriteId::xD0_LYNEL => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x14_FRIENDLY_LYNEL,
        ]],
        SpriteId::xD2_FLOPPING_FISH => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::xC_OCTOROK_ZORA,
            SpriteSheetId::None,
        ]],
        SpriteId::xD5_DIGGING_GAME_PROPRIETOR => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x2A_HAZARDS,
            SpriteSheetId::x2A_HAZARDS,
            SpriteSheetId::None,
        ]],
        SpriteId::xD6_GANON => vec![[
            SpriteSheetId::x21_BIG_BAD_GUY,
            SpriteSheetId::x41_BIG_BAD_GUY,
            SpriteSheetId::x45_BIG_BAD_GUY,
            SpriteSheetId::x33_BIG_BAD_GUY,
        ]],
        SpriteId::xD7_GANON_INVINCIBLE => vec![[
            SpriteSheetId::x21_BIG_BAD_GUY,
            SpriteSheetId::x41_BIG_BAD_GUY,
            SpriteSheetId::x45_BIG_BAD_GUY,
            SpriteSheetId::x33_BIG_BAD_GUY,
        ]],
        SpriteId::xE7_MUSHROOM => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x11_MISC_FAKE_SWORD,
        ]],
        SpriteId::xE8_FAKE_MASTER_SWORD => vec![[
            SpriteSheetId::None,
            SpriteSheetId::x49_SOLDIERS,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::xE9_MAGIC_MERCHANT => vec![[
            SpriteSheetId::x4B_ARCHERY,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::xED_SOMARIA_PLATFORM => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x27_TURTLE_ROCK,
            SpriteSheetId::None,
        ]],
        SpriteId::xEE_CASTLE_MANTLE => vec![[
            SpriteSheetId::x5D_MANTLE_CREDITS,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::xF2_MEDALLION_TABLET => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x12_DESERT_1,
            SpriteSheetId::None,
        ]],
        SpriteId::xF3_PERSONS_DOOR_OW_OVERLORD => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::xF4_FALLING_ROCKS_OW_OVERLORD => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::xBC_DRUNK_IN_THE_INN => vec![[
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
            SpriteSheetId::None,
        ]],
        SpriteId::xD4_LANDMINE => vec![],
        _ => vec![],
    }
}
