use strum_macros::Display;

use crate::zelda3::model::SpriteId;

// Describes a challenge rating for sprites within their respective
// Sprite Type. This is used to identify how challenging the Sprite
// is to the player versus sprites of the same type.
// Concretely, harder enemies have a higher score and lower enemies
// have a lower score. Less useful consumables have a higher rating
// than ones that are more useful to the player. This is used almost
// solely for the Balancing options and is not sourced from the game.
#[derive(Display, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum SpriteChallenge {
    X0None = 0,
    X1Easiest = 1,
    X2Easy = 2,
    X3Easier = 3,
    X4Medium = 4,
    X5Hard = 5,
    X6Harder = 6,
    X7Hardest = 7,
}

pub(crate) fn get_sprite_challenge(sprite_id: &SpriteId) -> SpriteChallenge {
    // This is probably slightly less efficient than having a large table, but it is
    // a lot easier to maintain splitting sprite difficulty into groups.
    match sprite_id {
        SpriteId::x0_RAVEN => SpriteChallenge::X3Easier,
        SpriteId::x1_VULTURE => SpriteChallenge::X3Easier,
        SpriteId::x11_HINOX => SpriteChallenge::X7Hardest,
        SpriteId::x12_MOBLIN => SpriteChallenge::X4Medium,
        SpriteId::x13_MINI_HELMASAUR => SpriteChallenge::X4Medium,
        SpriteId::x15_ANTIFAIRY => SpriteChallenge::X3Easier,
        SpriteId::x17_HOARDER => SpriteChallenge::X1Easiest,
        SpriteId::x18_MINI_MOLDORM => SpriteChallenge::X4Medium,
        SpriteId::x19_POE => SpriteChallenge::X2Easy,
        SpriteId::x2_STALFOS_HEAD => SpriteChallenge::X3Easier,
        SpriteId::x20_SLUGGULA => SpriteChallenge::X5Hard,
        SpriteId::x22_ROPA => SpriteChallenge::X3Easier,
        SpriteId::x23_RED_BARI => SpriteChallenge::X2Easy,
        SpriteId::x24_BLUE_BARI => SpriteChallenge::X2Easy,
        SpriteId::x26_HARDHAT_BEETLE => SpriteChallenge::X5Hard,
        SpriteId::x27_DEADROCK => SpriteChallenge::X5Hard,
        SpriteId::x3E_HOARDER_ROCK => SpriteChallenge::X1Easiest,
        SpriteId::x41BlueSwordGuard => SpriteChallenge::X5Hard,
        SpriteId::x42_GREEN_GUARD => SpriteChallenge::X3Easier,
        SpriteId::x43_RED_SPEAR_GUARD => SpriteChallenge::X6Harder,
        SpriteId::x44_BLUE_ASSAULT_GUARD => SpriteChallenge::X7Hardest,
        SpriteId::x45RedSpearGuard2 => SpriteChallenge::X7Hardest,
        SpriteId::x46_BLUE_ARCHER => SpriteChallenge::X5Hard,
        SpriteId::x47_GREEN_GUARD_BUSH => SpriteChallenge::X3Easier,
        SpriteId::x48_RED_JAVELIN_GUARD => SpriteChallenge::X5Hard,
        SpriteId::x49_RED_GUARD_BUSH => SpriteChallenge::X6Harder,
        SpriteId::x4A_RED_BOMB_GUARD => SpriteChallenge::X5Hard,
        SpriteId::x4B_GREEN_KNIFE_GUARD => SpriteChallenge::X1Easiest,
        SpriteId::x4C_GELDMAN => SpriteChallenge::X3Easier,
        SpriteId::x4E_POPO_1 => SpriteChallenge::X2Easy,
        SpriteId::x4F_POPO_2 => SpriteChallenge::X2Easy,
        SpriteId::x51_ARMOS_STATUE => SpriteChallenge::X4Medium,
        SpriteId::x53_ARMOS_KNIGHT => SpriteChallenge::X1Easiest,
        SpriteId::x54_LANMOLAS => SpriteChallenge::X1Easiest,
        SpriteId::x55_FIREBALL_ZORA => SpriteChallenge::X4Medium,
        SpriteId::x56_WALKING_ZORA => SpriteChallenge::X3Easier,
        SpriteId::x58_CRAB => SpriteChallenge::X3Easier,
        SpriteId::x5B_SPARK_CLOCKWISE => SpriteChallenge::X1Easiest,
        SpriteId::x5C_SPARK_COUNTER_CLOCKWISE => SpriteChallenge::X1Easiest,
        SpriteId::x5D_ROLLER_SOUTH => SpriteChallenge::X5Hard,
        SpriteId::x5E_ROLLER_NORTH => SpriteChallenge::X5Hard,
        SpriteId::x5F_ROLLER_EAST => SpriteChallenge::X5Hard,
        SpriteId::x60_ROLLER_WEST => SpriteChallenge::X5Hard,
        SpriteId::x61_BEAMOS => SpriteChallenge::X4Medium,
        SpriteId::x63_DEVALANT_PIT => SpriteChallenge::X5Hard,
        SpriteId::x64_DEVALANT => SpriteChallenge::X5Hard,
        SpriteId::x66_MOVING_CANNON_EAST => SpriteChallenge::X5Hard,
        SpriteId::x67_MOVING_CANNON_WEST => SpriteChallenge::X5Hard,
        SpriteId::x68_MOVING_CANNON_SOUTH => SpriteChallenge::X5Hard,
        SpriteId::x69_MOVING_CANNON_NORTH => SpriteChallenge::X5Hard,
        SpriteId::x6A_BALL_N_CHAIN_GUARD => SpriteChallenge::X7Hardest,
        SpriteId::x6B_CANNON_GUARD => SpriteChallenge::X3Easier,
        SpriteId::x6D_RAT_CRICKET => SpriteChallenge::X1Easiest,
        SpriteId::x6E_ROPE => SpriteChallenge::X1Easiest,
        SpriteId::x6F_KEESE => SpriteChallenge::X2Easy,
        SpriteId::x71_LEEVER => SpriteChallenge::X2Easy,
        SpriteId::x77_ANTIFAIRY_2 => SpriteChallenge::X7Hardest,
        SpriteId::x79_BEE => SpriteChallenge::X7Hardest,
        SpriteId::x7A_AGAHNIM => SpriteChallenge::X3Easier,
        SpriteId::x7C_FloatingStalfosHead => SpriteChallenge::X4Medium,
        SpriteId::x7D_BIG_SPIKE => SpriteChallenge::X7Hardest,
        SpriteId::x7E_FIREBAR_CLOCKWISE => SpriteChallenge::X4Medium,
        SpriteId::x7F_FIREBAR_COUNTER_CLOCKWISE => SpriteChallenge::X4Medium,
        SpriteId::x8_OCTOROK => SpriteChallenge::X2Easy,
        SpriteId::x80_FIRESNAKE => SpriteChallenge::X4Medium,
        SpriteId::x81_WATER_TEKTITE => SpriteChallenge::X3Easier,
        SpriteId::x82_ANTIFAIRY_CIRCLE => SpriteChallenge::X7Hardest,
        SpriteId::x83_GREEN_EYEGORE => SpriteChallenge::X4Medium,
        SpriteId::x84_RED_EYEGORE => SpriteChallenge::X5Hard,
        SpriteId::x85_YELLOW_STALFOS => SpriteChallenge::X5Hard,
        SpriteId::x86_KODONGO => SpriteChallenge::X4Medium,
        SpriteId::x88_MOTHULA => SpriteChallenge::X3Easier,
        SpriteId::x8A_SPIKE_BLOCK => SpriteChallenge::X6Harder,
        SpriteId::x8B_GIBDO => SpriteChallenge::X5Hard,
        SpriteId::x8C_ARRGHUS => SpriteChallenge::X4Medium,
        SpriteId::x8D_ARRGHUS_SPAWN => SpriteChallenge::X5Hard,
        SpriteId::x8E_TERRORPIN => SpriteChallenge::X4Medium,
        SpriteId::x8F_BLOB => SpriteChallenge::X3Easier,
        SpriteId::x9_MOLDORM => SpriteChallenge::X1Easiest,
        SpriteId::x91_STALFOS_KNIGHT => SpriteChallenge::X6Harder,
        SpriteId::x92_KING_HELMASAUR => SpriteChallenge::X4Medium,
        SpriteId::x93_BUMPER => SpriteChallenge::X2Easy,
        SpriteId::x94_PIROGUSU => SpriteChallenge::X5Hard,
        SpriteId::x95_EYE_LASER_EAST => SpriteChallenge::X5Hard,
        SpriteId::x96_EYE_LASER_WEST => SpriteChallenge::X5Hard,
        SpriteId::x97_EYE_LASER_SOUTH => SpriteChallenge::X5Hard,
        SpriteId::x98_EYE_LASER_NORTH => SpriteChallenge::X5Hard,
        SpriteId::x99_PENGATOR => SpriteChallenge::X4Medium,
        SpriteId::x9A_KYAMERON => SpriteChallenge::X4Medium,
        SpriteId::x9B_WIZZROBE => SpriteChallenge::X6Harder,
        SpriteId::x9C_BABASU_EAST => SpriteChallenge::X2Easy,
        SpriteId::x9D_BABUSU_SOUTH => SpriteChallenge::X2Easy,
        SpriteId::xA_OCTOROK_FOUR_WAY => SpriteChallenge::X2Easy,
        SpriteId::xA1_FREEZOR => SpriteChallenge::X5Hard,
        SpriteId::xA2_KHOLDSTARE => SpriteChallenge::X5Hard,
        SpriteId::xA4_FALLING_ICE => SpriteChallenge::X4Medium,
        SpriteId::xA5_BLUE_ZAZAK => SpriteChallenge::X3Easier,
        SpriteId::xA6_RED_ZAZAK => SpriteChallenge::X5Hard,
        SpriteId::xA7_STALFOS => SpriteChallenge::X3Easier,
        SpriteId::xA8_GREEN_ZIRRO => SpriteChallenge::X4Medium,
        SpriteId::xA9_BLUE_ZIRRO => SpriteChallenge::X5Hard,
        SpriteId::xAA_PIKIT_LIKE_LIKE => SpriteChallenge::X6Harder,
        SpriteId::xAC_APPLE => SpriteChallenge::X4Medium,
        SpriteId::xB8_Goriya => SpriteChallenge::X4Medium,
        SpriteId::xBD_VITREOUS => SpriteChallenge::X5Hard,
        SpriteId::xC3_GIBO => SpriteChallenge::X3Easier,
        SpriteId::xC4_THIEF => SpriteChallenge::X4Medium,
        SpriteId::xC5_MEDUSA => SpriteChallenge::X4Medium,
        SpriteId::xC6_MEDUSA_FOUR_WAY => SpriteChallenge::X5Hard,
        SpriteId::xC7_Hokkubokku_Pokey => SpriteChallenge::X6Harder,
        SpriteId::xC9_TEKTITE => SpriteChallenge::X3Easier,
        SpriteId::xCA_CHAIN_CHOMP => SpriteChallenge::X4Medium,
        SpriteId::xCB_TRINEXX_ROCK => SpriteChallenge::X6Harder,
        SpriteId::xCC_TRINEXX_FIRE => SpriteChallenge::X6Harder,
        SpriteId::xCD_TRINEXX_ICE => SpriteChallenge::X6Harder,
        SpriteId::xCE_BLIND => SpriteChallenge::X4Medium,
        SpriteId::xCF_SWAMOLA => SpriteChallenge::X2Easy,
        SpriteId::xD_BUZZBLOB => SpriteChallenge::X2Easy,
        SpriteId::xD0_LYNEL => SpriteChallenge::X7Hardest,
        SpriteId::xD1_BUNNY_BEAM => SpriteChallenge::X3Easier,
        SpriteId::xD3_STAL => SpriteChallenge::X0None,
        SpriteId::xD4_LANDMINE => SpriteChallenge::X4Medium,
        SpriteId::xD6_GANON => SpriteChallenge::X7Hardest,
        SpriteId::xD8_HEART => SpriteChallenge::X7Hardest,
        SpriteId::xD9_GREEN_RUPEE => SpriteChallenge::X7Hardest,
        SpriteId::xDA_BLUE_RUPEE => SpriteChallenge::X4Medium,
        SpriteId::xDB_RED_RUPEE => SpriteChallenge::X1Easiest,
        SpriteId::xDC_BOMB_REFILL_1 => SpriteChallenge::X7Hardest,
        SpriteId::xDD_BOMB_REFILL_4 => SpriteChallenge::X4Medium,
        SpriteId::xDE_BOMB_REFILL_8 => SpriteChallenge::X1Easiest,
        SpriteId::xDF_SMALL_MAGIC_REFILL => SpriteChallenge::X4Medium,
        SpriteId::xE_SNAPDRAGON => SpriteChallenge::X4Medium,
        SpriteId::xE0_FULL_MAGIC_REFILL => SpriteChallenge::X1Easiest,
        SpriteId::xE1_ARROW_REFILL_5 => SpriteChallenge::X4Medium,
        SpriteId::xE2_ARROW_REFILL_10 => SpriteChallenge::X1Easiest,
        SpriteId::xE3_FAIRY => SpriteChallenge::X1Easiest,
        SpriteId::xF_OCTOBALLOON => SpriteChallenge::X2Easy,
        SpriteId::x4D_TOPPO => SpriteChallenge::X1Easiest,
        SpriteId::x59_LOST_WOODS_BIRD => SpriteChallenge::X1Easiest,
        SpriteId::x5A_LOST_WOODS_SQUIRREL => SpriteChallenge::X1Easiest,
        SpriteId::x9E_HAUNTED_GROVE_OSTRICH => SpriteChallenge::X1Easiest,
        SpriteId::x9F_HAUNTED_GROVE_RABBIT => SpriteChallenge::X1Easiest,
        SpriteId::xA0_HAUNTED_GROVE_BIRD => SpriteChallenge::X1Easiest,
        SpriteId::xB2_GOOD_BEE => SpriteChallenge::X2Easy,
        SpriteId::xD2_FLOPPING_FISH => SpriteChallenge::X1Easiest,
        _ => SpriteChallenge::X0None,
    }
}
