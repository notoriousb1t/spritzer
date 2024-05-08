use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::FromRepr;

/// Describes Sprites (aka Entities) in ALTTP.
/// This is set as a u16 with overlords included, but overlords should
/// be split so that this fits in a u8 and because overlords and sprites
/// are functionally very different. Possibly use a separate enum?
#[repr(u16)]
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Hash, FromRepr, EnumIter, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub(crate) enum SpriteId {
    x0_RAVEN = 0,
    x1_VULTURE = 1,
    x2_STALFOS_HEAD = 2,
    x3_NONE = 3,
    x4_PULL_SWITCH_NORMAL = 4,
    x5_PULL_SWITCH_NORMAL_UNUSED = 5,
    x6_PULL_SWITCH_TRAP = 6,
    x7_PULL_SWITCH_TRAP_UNUSED = 7,
    x8_OCTOROK = 8,
    x9_MOLDORM = 9,
    xA_OCTOROK_FOUR_WAY = 10,
    xB_CUCCO = 11,
    xC_OCTOROK_STONE = 12,
    xD_BUZZBLOB = 13,
    xE_SNAPDRAGON = 14,
    xF_OCTOBALLOON = 15,
    x10_OCTOBALLOON_HATCHLINGS = 16,
    x11_HINOX = 17,
    x12_MOBLIN = 18,
    x13_MINI_HELMASAUR = 19,
    x14_THIEVES_TOWN_GRATE = 20,
    x15_ANTIFAIRY = 21,
    x16_SAHASRAHLA = 22,
    x17_HOARDER = 23,
    x18_MINI_MOLDORM = 24,
    x19_POE = 25,
    x1A_SMITHY = 26,
    x1B_ARROW = 27,
    x1C_STATUE = 28,
    x1D_FLUTEQUEST = 29,
    x1E_CRYSTAL_SWITCH = 30,
    x1F_BUG_CATCHING_KID = 31,
    x20_SLUGGULA = 32,
    x21_WATER_SWITCH = 33,
    x22_ROPA = 34,
    x23_RED_BARI = 35,
    x24_BLUE_BARI = 36,
    x25_TALKING_TREE = 37,
    x26_HARDHAT_BEETLE = 38,
    x27_DEADROCK = 39,
    x28_HINT_PC_DW = 40,
    x29_BLIND_HIDEOUT_ATTENDANT = 41,
    x2A_SWEEPING_LADY = 42,
    x2B_TENTMAN = 43,
    x2C_LUMBERJACKS = 44,
    x2D_NECKLESS_MAN = 45,
    x2E_FLUTE_KID = 46,
    x2F_RACE_LADY = 47,
    x30_RACE_GUY = 48,
    x31_FORTUNE_TELLER = 49,
    x32_ANGRY_BROTHERS = 50,
    x33_RUPEE_PULL = 51,
    x34_SNITCH_YOUNG = 52,
    x35_INNKEEPER = 53,
    x36_WITCH = 54,
    x37_WATERFALL = 55,
    x38_EYEGORE_STATUE = 56,
    x39_LOCKSMITH = 57,
    x3A_MAGIC_BAT = 58,
    x3B_BONK_ITEM = 59,
    x3C_VILLAGE_KID = 60,
    x3D_SNITCH_OLD = 61,
    x3E_HOARDER_ROCK = 62,
    x3F_TUTORIAL_SOLDIER = 63,
    x40_LIGHTNING_LOCK = 64,
    x41BlueSwordGuard = 65,
    x42_GREEN_GUARD = 66,
    x43_RED_SPEAR_GUARD = 67,
    x44_BLUE_ASSAULT_GUARD = 68,
    x45RedSpearGuard2 = 69,
    x46_BLUE_ARCHER = 70,
    x47_GREEN_GUARD_BUSH = 71,
    x48_RED_JAVELIN_GUARD = 72,
    x49_RED_GUARD_BUSH = 73,
    x4A_RED_BOMB_GUARD = 74,
    x4B_GREEN_KNIFE_GUARD = 75,
    x4C_GELDMAN = 76,
    x4D_TOPPO = 77,
    x4E_POPO_1 = 78,
    x4F_POPO_2 = 79,
    x50_CANNON_BALL = 80,
    x51_ARMOS_STATUE = 81,
    x52_ZORA_KING = 82,
    x53_ARMOS_KNIGHT = 83,
    x54_LANMOLAS = 84,
    x55_FIREBALL_ZORA = 85,
    x56_WALKING_ZORA = 86,
    x57_DESERT_STATUE = 87,
    x58_CRAB = 88,
    x59_LOST_WOODS_BIRD = 89,
    x5A_LOST_WOODS_SQUIRREL = 90,
    x5B_SPARK_CLOCKWISE = 91,
    x5C_SPARK_COUNTER_CLOCKWISE = 92,
    x5D_ROLLER_SOUTH = 93,
    x5E_ROLLER_NORTH = 94,
    x5F_ROLLER_EAST = 95,
    x60_ROLLER_WEST = 96,
    x61_BEAMOS = 97,
    x62_MASTERSWORD = 98,
    x63_DEVALANT_PIT = 99,
    x64_DEVALANT = 100,
    x65_ARCHERY_GUY = 101,
    x66_MOVING_CANNON_EAST = 102,
    x67_MOVING_CANNON_WEST = 103,
    x68_MOVING_CANNON_SOUTH = 104,
    x69_MOVING_CANNON_NORTH = 105,
    x6A_BALL_N_CHAIN_GUARD = 106,
    x6B_CANNON_GUARD = 107,
    x6C_MIRROR_PORTAL = 108,
    x6D_RAT_CRICKET = 109,
    x6E_ROPE = 110,
    x6F_KEESE = 111,
    x70_HELMASAUR_KING_FIREBALL = 112,
    x71_LEEVER = 113,
    x72_FAIRY_POND_TRIGGER = 114,
    x73_UNCLE_PRIEST_MANTLE = 115,
    x74_RUNNING_MAN = 116,
    x75_BOTTLE_SALESMAN = 117,
    x76_ZELDA = 118,
    x77_ANTIFAIRY_2 = 119,
    x78_VILLAGE_ELDER = 120,
    x79_BEE = 121,
    x7A_AGAHNIM = 122,
    x7B_AGAHNIM_ENERGY_BALL = 123,
    x7C_FloatingStalfosHead = 124,
    x7D_BIG_SPIKE = 125,
    x7E_FIREBAR_CLOCKWISE = 126,
    x7F_FIREBAR_COUNTER_CLOCKWISE = 127,
    x80_FIRESNAKE = 128,
    x81_WATER_TEKTITE = 129,
    x82_ANTIFAIRY_CIRCLE = 130,
    x83_GREEN_EYEGORE = 131,
    x84_RED_EYEGORE = 132,
    x85_YELLOW_STALFOS = 133,
    x86_KODONGO = 134,
    x87_KODONGO_FIRE = 135,
    x88_MOTHULA = 136,
    x89_MOTHULA_BEAM = 137,
    x8A_SPIKE_BLOCK = 138,
    x8B_GIBDO = 139,
    x8C_ARRGHUS = 140,
    x8D_ARRGHUS_SPAWN = 141,
    x8E_TERRORPIN = 142,
    x8F_BLOB = 143,
    x90_WALLMASTER = 144,
    x91_STALFOS_KNIGHT = 145,
    x92_KING_HELMASAUR = 146,
    x93_BUMPER = 147,
    x94_PIROGUSU = 148,
    x95_EYE_LASER_EAST = 149,
    x96_EYE_LASER_WEST = 150,
    x97_EYE_LASER_SOUTH = 151,
    x98_EYE_LASER_NORTH = 152,
    x99_PENGATOR = 153,
    x9A_KYAMERON = 154,
    x9B_WIZZROBE = 155,
    x9C_BABASU_EAST = 156,
    x9D_BABUSU_SOUTH = 157,
    x9E_HAUNTED_GROVE_OSTRICH = 158,
    x9F_HAUNTED_GROVE_RABBIT = 159,
    xA0_HAUNTED_GROVE_BIRD = 160,
    xA1_FREEZOR = 161,
    xA2_KHOLDSTARE = 162,
    xA3_KHOLDSTARES_SHELL = 163,
    xA4_FALLING_ICE = 164,
    xA5_BLUE_ZAZAK = 165,
    xA6_RED_ZAZAK = 166,
    xA7_STALFOS = 167,
    xA8_GREEN_ZIRRO = 168,
    xA9_BLUE_ZIRRO = 169,
    xAA_PIKIT_LIKE_LIKE = 170,
    xAB_CRYSTAL_MAIDEN = 171,
    xAC_APPLE = 172,
    xAD_LOST_OLD_MAN = 173,
    xAE_DOWN_PIPE = 174,
    xAF_UP_PIPE = 175,
    xB0_RIGHT_PIPE = 176,
    xB1_LEFT_PIPE = 177,
    xB2_GOOD_BEE = 178,
    xB3_PEDESTAL_PLAQUE = 179,
    xB4_PURPLE_CHEST = 180,
    xB5_BOMB_SALESMAN = 181,
    xB6_KIKI = 182,
    xB7_BLIND_MAIDEN = 183,
    xB8_Goriya = 184,
    xB9_BULLY_AND_FRIEND = 185,
    xBA_WHIRLPOOL = 186,
    xBB_SALESMAN = 187,
    xBC_DRUNK_IN_THE_INN = 188,
    xBD_VITREOUS = 189,
    xBE_VITREOUS_SMALL_EYEBALL = 190,
    xBF_LIGHTNING = 191,
    xC0_CATFISH = 192,
    xC1_AGAHNIM_TELEPORTING = 193,
    xC2_BOULDER = 194,
    xC3_GIBO = 195,
    xC4_THIEF = 196,
    xC5_MEDUSA = 197,
    xC6_MEDUSA_FOUR_WAY = 198,
    xC7_Hokkubokku_Pokey = 199,
    xC8_GREAT_FAIRY = 200,
    xC9_TEKTITE = 201,
    xCA_CHAIN_CHOMP = 202,
    xCB_TRINEXX_ROCK = 203,
    xCC_TRINEXX_FIRE = 204,
    xCD_TRINEXX_ICE = 205,
    xCE_BLIND = 206,
    xCF_SWAMOLA = 207,
    xD0_LYNEL = 208,
    xD1_BUNNY_BEAM = 209,
    xD2_FLOPPING_FISH = 210,
    xD3_STAL = 211,
    xD4_LANDMINE = 212,
    xD5_DIGGING_GAME_PROPRIETOR = 213,
    xD6_GANON = 214,
    xD7_GANON_INVINCIBLE = 215,
    xD8_HEART = 216,
    xD9_GREEN_RUPEE = 217,
    xDA_BLUE_RUPEE = 218,
    xDB_RED_RUPEE = 219,
    xDC_BOMB_REFILL_1 = 220,
    xDD_BOMB_REFILL_4 = 221,
    xDE_BOMB_REFILL_8 = 222,
    xDF_SMALL_MAGIC_REFILL = 223,
    xE0_FULL_MAGIC_REFILL = 224,
    xE1_ARROW_REFILL_5 = 225,
    xE2_ARROW_REFILL_10 = 226,
    xE3_FAIRY = 227,
    xE4_SMALL_KEY = 228,
    xE5_BIG_KEY = 229,
    xE6_SHIELD = 230,
    xE7_MUSHROOM = 231,
    xE8_FAKE_MASTER_SWORD = 232,
    xE9_MAGIC_MERCHANT = 233,
    xEA_HEART_CONTAINER = 234,
    xEB_HEART_PIECE = 235,
    xEC_THROWN_ITEM = 236,
    xED_SOMARIA_PLATFORM = 237,
    xEE_CASTLE_MANTLE = 238,
    xEF_SOMARIA_PLATFORM_UNUSED_1 = 239,
    xF0_SOMARIA_PLATFORM_UNUSED_2 = 240,
    xF1_SOMARIA_PLATFORM_UNUSED_3 = 241,
    xF2_MEDALLION_TABLET = 242,

    // Overworld overlords.
    xF3_PERSONS_DOOR_OW_OVERLORD = 243,
    xF4_FALLING_ROCKS_OW_OVERLORD = 244,
    xF5_CANON_BALLS_OW_OVERLORD = 245,
    xF6_UNKNOWN_F6_OW_OVERLORD = 246,
    xF7_UNKNOWN_F7_OW_OVERLORD = 247,
    xF8_UNKNOWN_F8_OW_OVERLORD = 248,
    xF9_UNKNOWN_F9_OW_OVERLORD = 249,
    xFA_BLOB_DROP_OW_OVERLORD = 250,
    xFB_OVERWORLD_WALLMASTER_OW_OVERLORD_DROPS_IN_HOULIHAN = 251,
    xFC_FLOOR_DROP_SQUARE_OW_OVERLORD = 252,
    xFD_FLOOR_DROP_NORTH_PATH_OW_OVERLORD = 253,
    xFE_FLOOR_DROP_EAST_PATH_OW_OVERLORD = 254,

    // Underworld overlords.
    x102_CANON_BALLS_EP_4_WALL_CANONBALLS = 258,
    x103_CANON_BALLS_EP_ENTRY = 259,
    x104_ROPE_DROP_TRAP = 260,
    x105_STALFOS_HEAD_TRAP = 261,
    x106_BOMB_DROP1_TRAP = 262,
    x107_MOVING_FLOOR = 263,
    x108_TRANSFORMER_BUNNY_BEAM = 264,
    x109_WALLMASTER_OVERLORD = 265,
    x10A_FLOOR_DROP_SQUARE = 266,
    x10B_FLOOR_DROP_NORTH = 267,
    x110_PIROGUSU_SPAWNER_RIGHT = 272,
    x111_PIROGUSU_SPAWNER_LEFT = 273,
    x112_PIROGUSU_SPAWNER_DOWN = 274,
    x113_PIROGUSU_SPAWNER_UP = 275,
    x114_FLYING_FLOOR_TILE_TRAP = 276,
    x115_WIZZROBE_SPAWNER = 277,
    x116_ZORO_SPAWNER = 278,
    x117_FOUR_SKULL_TRAP_IN_POD_UNDER_POT = 279,
    x118_STALFOS_APPEAR = 280,
    x119_ARMOS_KNIGHTS_TRIGGER = 281,
    x11A_BOMB_DROP2_TRAP = 282,
    x141_SOLDIER_ALERTER_BLUE = 321,
    x142_SOLDIER_ALERTER_GREEN = 322,
}