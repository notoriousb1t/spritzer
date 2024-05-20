use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::FromRepr;

/// An identifier for a room in dungeons, houses, and caves.
#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Hash, FromRepr, EnumIter, PartialOrd, Ord)]
pub(crate) enum UWRoomId {
    x00_GANON = 0,
    x01_HYRULE_CASTLE_NORTH_CORRIDOR = 1,
    x02_HYRULE_CASTLE_SWITCH_ROOM = 2,
    x03_HOULIHAN_ROOM = 3,
    x04_TURTLE_ROCK_CRYSTA_ROLLER_ROOM = 4,
    x05_EMPTY_CLONE_ROOM1 = 5,
    x06_SWAMP_PALACE_ARRGHUS_BOSS = 6,
    x07_TOWER_OF_HERA_MOLDORM_BOSS = 7,
    x08_CAVE_HEALING_FAIRY = 8,
    x09_PALACE_OF_DARKNESS = 9,
    x0A_PALACE_OF_DARKNESS_STALFOS_TRAP_ROOM = 10,
    x0B_PALACE_OF_DARKNESS_TURTLE_ROOM = 11,
    x0C_GANON_S_TOWER_ENTRANCE_ROOM = 12,
    x0D_GANON_S_TOWER_AGAHNIM2_BOSS = 13,
    x0E_ICE_PALACE_ENTRANCE_ROOM = 14,
    x0F_EMPTY_CLONE_ROOM2 = 15,
    x10_GANON_EVACUATION_ROUTE = 16,
    x11_HYRULE_CASTLE_BOMBABLE_STOCK_ROOM = 17,
    x12_SANCTUARY = 18,
    x13_TURTLE_ROCK_HOKKU_BOKKU_KEY_ROOM_2 = 19,
    x14_TURTLE_ROCK_BIG_KEY_ROOM = 20,
    x15_TURTLE_ROCK = 21,
    x16_SWAMP_PALACE_SWIMMING_TREADMILL = 22,
    x17_TOWER_OF_HERA_MOLDORM_FALL_ROOM = 23,
    x18_BIG_FAIRY_DROP_ENTRANCE_CAVE = 24,
    x19_PALACE_OF_DARKNESS_DARK_MAZE = 25,
    x1A_PALACE_OF_DARKNESS_BIG_CHEST_ROOM = 26,
    x1B_PALACE_OF_DARKNESS_MIMICS_MOVING_WALL_ROOM = 27,
    x1C_GANON_S_TOWER_ICE_ARMOS = 28,
    x1D_GANON_S_TOWER_FINAL_HALLWAY = 29,
    x1E_ICE_PALACE_BOMB_FLOOR_BARI_ROOM = 30,
    x1F_ICE_PALACE_PENGATOR_BIG_KEY_ROOM = 31,
    x20_AGAHNIM_S_TOWER_AGAHNIM_BOSS = 32,
    x21_HYRULE_CASTLE_KEY_RAT_ROOM = 33,
    x22_HYRULE_CASTLE_SEWER_TEXT_TRIGGER_ROOM = 34,
    x23_TURTLE_ROCK_WEST_EXIT_TO_BALCONY = 35,
    x24_TURTLE_ROCK_DOUBLE_HOKKU_BOKKU_BIG_CHEST_ROOM = 36,
    x25_EMPTY_CLONE_ROOM3 = 37,
    x26_SWAMP_PALACE_STATUE_ROOM = 38,
    x27_TOWER_OF_HERA_BIG_CHEST = 39,
    x28_SWAMP_PALACE_ENTRANCE_ROOM = 40,
    x29_SKULL_WOODS_MOTHULA_BOSS = 41,
    x2A_PALACE_OF_DARKNESS_BIG_HUB_ROOM = 42,
    x2B_PALACE_OF_DARKNESS_MAP_CHEST_FAIRY_ROOM = 43,
    x2C_HOOKSHOT_CAVE_BACKDOOR_BIG_FAIRY = 44,
    x2D_EMPTY_CLONE_ROOM = 45,
    x2E_ICE_PALACE_COMPASS_ROOM = 46,
    x2F_CAVE_KAKARIKO_WELL_HP = 47,
    x30_AGAHNIM_S_TOWER_MAIDEN_SACRIFICE_CHAMBER = 48,
    x31_TOWER_OF_HERA_HARDHAT_BEETLES_ROOM = 49,
    x32_HYRULE_CASTLE_SEWER_KEY_CHEST_ROOM = 50,
    x33_DESERT_PALACE_LANMOLAS_BOSS = 51,
    x34_SWAMP_PALACE_PUSH_BLOCK_PUZZLE_PRE_BIG_KEY_ROOM = 52,
    x35_SWAMP_PALACE_BIG_KEY_BS_ROOM = 53,
    x36_SWAMP_PALACE_BIG_CHEST_ROOM = 54,
    x37_SWAMP_PALACE_MAP_CHEST_WATER_FILL_ROOM = 55,
    x38_SWAMP_PALACE_KEY_POT_ROOM = 56,
    x39_SKULL_WOODS_GIBDO_KEY_MOTHULA_HOLE_ROOM = 57,
    x3A_PALACE_OF_DARKNESS_BOMBABLE_FLOOR_ROOM = 58,
    x3B_PALACE_OF_DARKNESS_SPIKE_BLOCK_CONVEYOR_ROOM = 59,
    x3C_HOOKSHOT_CAVE = 60,
    x3D_GANON_S_TOWER_TORCH_ROOM_2 = 61,
    x3E_ICE_PALACE_STALFOS_KNIGHTS_CONVEYOR_HELLWAY = 62,
    x3F_ICE_PALACE_MAP_CHEST_ROOM = 63,
    x40_AGAHNIM_S_TOWER_FINAL_BRIDGE_ROOM = 64,
    x41_HYRULE_CASTLE_FIRST_DARK_ROOM = 65,
    x42_HYRULE_CASTLE_6_ROPES_ROOM = 66,
    x43_DESERT_PALACE_TORCH_PUZZLE_MOVING_WALL_ROOM = 67,
    x44_THIEVES_TOWN_BIG_CHEST_ROOM = 68,
    x45_THIEVES_TOWN_JAIL_CELLS_ROOM = 69,
    x46_SWAMP_PALACE_COMPASS_CHEST_ROOM = 70,
    x47_EMPTY_CLONE_ROOM4 = 71,
    x48_EMPTY_CLONE_ROOM5 = 72,
    x49_SKULL_WOODS_GIBDO_TORCH_PUZZLE_ROOM = 73,
    x4A_PALACE_OF_DARKNESS_ENTRANCE_ROOM = 74,
    x4B_PALACE_OF_DARKNESS_WARPS_SOUTH_MIMICS_ROOM = 75,
    x4C_GANON_S_TOWER_MINI_HELMASAUR_CONVEYOR_ROOM = 76,
    x4D_GANON_S_TOWER_MOLDORM_ROOM = 77,
    x4E_ICE_PALACE_BOMB_JUMP_ROOM = 78,
    x4F_ICE_PALACE_CLONE_ROOM_FAIRY_ROOM = 79,
    x50_HYRULE_CASTLE_WEST_CORRIDOR = 80,
    x51_HYRULE_CASTLE_THRONE_ROOM = 81,
    x52_HYRULE_CASTLE_EAST_CORRIDOR = 82,
    x53_DESERT_PALACE_POPOS_2_BEAMOS_HELLWAY_ROOM = 83,
    x54_SWAMP_PALACE_UPSTAIRS_PITS_ROOM = 84,
    x55_CASTLE_SECRET_ENTRANCE_UNCLE_DEATH_ROOM = 85,
    x56_SKULL_WOODS_KEY_POT_TRAP_ROOM = 86,
    x57_SKULL_WOODS_BIG_KEY_ROOM = 87,
    x58_SKULL_WOODS_BIG_CHEST_ROOM = 88,
    x59_SKULL_WOODS_FINAL_SECTION_ENTRANCE_ROOM = 89,
    x5A_PALACE_OF_DARKNESS_HELMASAUR_KING_BOSS = 90,
    x5B_GANON_S_TOWER_SPIKE_PIT_ROOM = 91,
    x5C_GANON_S_TOWER_GANON_BALL_Z = 92,
    x5D_GANON_S_TOWER_GAUNTLET_1_2_3 = 93,
    x5E_ICE_PALACE_LONELY_FIREBAR = 94,
    x5F_ICE_PALACE_HIDDEN_CHEST_SPIKE_FLOOR_ROOM = 95,
    x60_HYRULE_CASTLE_WEST_ENTRANCE_ROOM = 96,
    x61_HYRULE_CASTLE_MAIN_ENTRANCE_ROOM = 97,
    x62_HYRULE_CASTLE_EAST_ENTRANCE_ROOM = 98,
    x63_DESERT_PALACE_FINAL_SECTION_ENTRANCE_ROOM = 99,
    x64_THIEVES_TOWN_WEST_ATTIC_ROOM = 100,
    x65_THIEVES_TOWN_EAST_ATTIC_ROOM = 101,
    x66_SWAMP_PALACE_HIDDEN_CHEST_HIDDEN_DOOR_ROOM = 102,
    x67_SKULL_WOODS_COMPASS_CHEST_ROOM = 103,
    x68_SKULL_WOODS_KEY_CHEST_TRAP_ROOM = 104,
    x69_EMPTY_CLONE_ROOM6 = 105,
    x6A_PALACE_OF_DARKNESS_RUPEE_ROOM = 106,
    x6B_GANON_S_TOWER_MIMICS_ROOMS = 107,
    x6C_GANON_S_TOWER_LANMOLAS_ROOM = 108,
    x6D_GANON_S_TOWER_GAUNTLET_4_5 = 109,
    x6E_ICE_PALACE_PENGATORS_ROOM = 110,
    x6F_EMPTY_CLONE_ROOM7 = 111,
    x70_HYRULE_CASTLE_SMALL_CORRIDOR_TO_JAIL_CELLS = 112,
    x71_HYRULE_CASTLE_BOOMERANG_CHEST_ROOM = 113,
    x72_HYRULE_CASTLE_MAP_CHEST_ROOM = 114,
    x73_DESERT_PALACE_BIG_CHEST_ROOM = 115,
    x74_DESERT_PALACE_MAP_CHEST_ROOM = 116,
    x75_DESERT_PALACE_BIG_KEY_CHEST_ROOM = 117,
    x76_SWAMP_PALACE_WATER_DRAIN_ROOM = 118,
    x77_TOWER_OF_HERA_ENTRANCE_ROOM = 119,
    x78_EMPTY_CLONE_ROOM8 = 120,
    x79_EMPTY_CLONE_ROOM9 = 121,
    x7A_EMPTY_CLONE_ROOM10 = 122,
    x7B_GANON_S_TOWER_SIDWAYS_CONVEYORS_4_CHEST_4_SHOOTER_SQUARE_PIT = 123,
    x7C_GANON_S_TOWER_EAST_SIDE_COLLAPSING_BRIDGE_EXPLODING_WALL_ROOM = 124,
    x7D_GANON_S_TOWER_WINDER_WARP_MAZE_ROOM = 125,
    x7E_ICE_PALACE_HIDDEN_CHEST_BOMBABLE_FLOOR_ROOM = 126,
    x7F_ICE_PALACE_BIG_SPIKE_TRAPS_ROOM = 127,
    x80_HYRULE_CASTLE_JAIL_CELL_ROOM = 128,
    x81_HYRULE_CASTLE_NEXT_TO_CHASM_ROOM = 129,
    x82_HYRULE_CASTLE_BASEMENT_CHASM_ROOM = 130,
    x83_DESERT_PALACE_WEST_ENTRANCE_ROOM = 131,
    x84_DESERT_PALACE_MAIN_ENTRANCE_ROOM = 132,
    x85_DESERT_PALACE_EAST_ENTRANCE_ROOM = 133,
    x86_EMPTY_CLONE_ROOM11 = 134,
    x87_TOWER_OF_HERA_TILE_ROOM = 135,
    x88_EMPTY_CLONE_ROOM12 = 136,
    x89_EASTERN_PALACE_FAIRY_ROOM = 137,
    x8A_EMPTY_CLONE_ROOM13 = 138,
    x8B_GANON_S_TOWER_BLOCK_PUZZLE_SPIKE_SKIP_MAP_CHEST_ROOM = 139,
    x8C_GANON_S_TOWER_EAST_AND_WEST_DOWNSTAIRS_BIG_CHEST_ROOM = 140,
    x8D_GANON_S_TOWER_TILE_TORCH_PUZZLE_ROOM = 141,
    x8E_ICE_PALACE_BLOBS_WITH_TETRIS_BARRIER = 142,
    x8F_EMPTY_CLONE_ROOM14 = 143,
    x90_MISERY_MIRE_VITREOUS_BOSS = 144,
    x91_MISERY_MIRE_FINAL_SWITCH_ROOM = 145,
    x92_MISERY_MIRE_DARK_BOMB_WALL_SWITCHES_ROOM = 146,
    x93_MISERY_MIRE_DARK_CANE_FLOOR_SWITCH_PUZZLE_ROOM = 147,
    x94_EMPTY_CLONE_ROOM15 = 148,
    x95_GANON_S_TOWER_FINAL_COLLAPSING_BRIDGE_ROOM = 149,
    x96_GANON_S_TOWER_TORCHES_1_ROOM = 150,
    x97_MISERY_MIRE_TORCH_PUZZLE_MOVING_WALL_ROOM = 151,
    x98_MISERY_MIRE_ENTRANCE_ROOM = 152,
    x99_EASTERN_PALACE_EYEGORE_KEY_ROOM = 153,
    x9A_EMPTY_CLONE_ROOM16 = 154,
    x9B_GANON_S_TOWER_MANY_SPIKES_WARP_MAZE_ROOM = 155,
    x9C_GANON_S_TOWER_INVISIBLE_FLOOR_MAZE_ROOM = 156,
    x9D_GANON_S_TOWER_COMPASS_CHEST_INVISIBLE_FLOOR_ROOM = 157,
    x9E_ICE_PALACE_BIG_CHEST_ROOM = 158,
    x9F_ICE_PALACE_ROOM_WITH_ICE_FLOOR_KEY_AND_4_WALL_RATS = 159,
    xA0_MISERY_MIRE_PRE_VITREOUS_ROOM = 160,
    xA1_MISERY_MIRE_FISH_ROOM = 161,
    xA2_MISERY_MIRE_BRIDGE_KEY_CHEST_ROOM = 162,
    xA3_MISERY_MIRE_EMPTY_L_CONNECTING_ROOM = 163,
    xA4_TURTLE_ROCK_TRINEXX_BOSS = 164,
    xA5_GANON_S_TOWER_WIZZROBES_ROOMS = 165,
    xA6_GANON_S_TOWER_MOLDORM_FALL_ROOM = 166,
    xA7_TOWER_OF_HERA_FAIRY_ROOM = 167,
    xA8_EASTERN_PALACE_STALFOS_SPAWN_ROOM = 168,
    xA9_EASTERN_PALACE_BIG_CHEST_ROOM = 169,
    xAA_EASTERN_PALACE_MAP_CHEST_ROOM = 170,
    xAB_THIEVES_TOWN_MOVING_SPIKES_KEY_POT_ROOM = 171,
    xAC_THIEVES_TOWN_BLIND_THE_THIEF_BOSS = 172,
    xAD_EMPTY_CLONE_ROOM17 = 173,
    xAE_ICE_PALACE_2_BLUE_BARI_AND_HIDDEN_CHEST = 174,
    xAF_ICE_PALACE_ICE_BRIDGE_ROOM = 175,
    xB0_AGAHNIM_S_TOWER_CIRCLE_OF_POTS = 176,
    xB1_MISERY_MIRE_HOURGLASS_ROOM = 177,
    xB2_MISERY_MIRE_SLUG_ROOM = 178,
    xB3_MISERY_MIRE_SPIKE_KEY_CHEST_ROOM = 179,
    xB4_TURTLE_ROCK_PRE_TRINEXX_ROOM = 180,
    xB5_TURTLE_ROCK_DARK_MAZE = 181,
    xB6_TURTLE_ROCK_CHAIN_CHOMPS_ROOM = 182,
    xB7_TURTLE_ROCK_MAP_CHEST_KEY_CHEST_ROLLER_ROOM = 183,
    xB8_EASTERN_PALACE_BIG_KEY_ROOM = 184,
    xB9_EASTERN_PALACE_LOBBY_CANNONBALLS_ROOM = 185,
    xBA_EASTERN_PALACE_DARK_ANTIFAIRY_KEY_POT_ROOM = 186,
    xBB_THIEVES_TOWN_HELLWAY = 187,
    xBC_THIEVES_TOWN_CONVEYOR_TOILET = 188,
    xBD_EMPTY_CLONE_ROOM18 = 189,
    xBE_ICE_PALACE_BLOCK_PUZZLE_ROOM = 190,
    xBF_ICE_PALACE_CLONE_ROOM_SWITCH_ROOM = 191,
    xC0_AGAHNIM_S_TOWER_DARK_BRIDGE_ROOM = 192,
    xC1_MISERY_MIRE_COMPASS_CHEST_TILE_ROOM = 193,
    xC2_MISERY_MIRE_BIG_HUB_ROOM = 194,
    xC3_MISERY_MIRE_BIG_CHEST_ROOM = 195,
    xC4_TURTLE_ROCK_FINAL_CRYSTAL_SWITCH_PUZZLE_ROOM = 196,
    xC5_TURTLE_ROCK_LASER_BRIDGE = 197,
    xC6_TURTLE_ROCK_AFTER_ENTRANCE_SQUARE_SOMARIA_PLATFORM_ROOM = 198,
    xC7_TURTLE_ROCK_TORCH_PUZZLE = 199,
    xC8_EASTERN_PALACE_ARMOS_KNIGHTS_BOSS = 200,
    xC9_EASTERN_PALACE_ENTRANCE_ROOM = 201,
    xCA_UNUSED_ROOM = 202,
    xCB_THIEVES_TOWN_NORTH_WEST_ENTRANCE_ROOM = 203,
    xCC_THIEVES_TOWN_NORTH_EAST_ENTRANCE_ROOM = 204,
    xCD_EMPTY_CLONE_ROOM19 = 205,
    xCE_ICE_PALACE_HOLE_TO_KHOLDSTARE_ROOM = 206,
    xCF_EMPTY_CLONE_ROOM20 = 207,
    xD0_AGAHNIM_S_TOWER_DARK_MAZE = 208,
    xD1_MISERY_MIRE_CONVEYOR_SLUG_BIG_KEY_ROOM = 209,
    xD2_MISERY_MIRE_MIRE02_WIZZROBES_ROOM = 210,
    xD3_EMPTY_CLONE_ROOM21 = 211,
    xD4_EMPTY_CLONE_ROOM22 = 212,
    xD5_TURTLE_ROCK_LASER_KEY_ROOM = 213,
    xD6_TURTLE_ROCK_ENTRANCE_ROOM = 214,
    xD7_EMPTY_CLONE_ROOM23 = 215,
    xD8_EASTERN_PALACE_ZELDAGAMER_ROOM_PRE_ARMOS_KNIGHTS_ROOM = 216,
    xD9_EASTERN_PALACE_CANONBALL_ROOM = 217,
    xDA_EASTERN_PALACE_2_BUBBLE_WITH_SWITCH_UNDER_POT = 218,
    xDB_THIEVES_TOWN_MAIN_SOUTH_WEST_ENTRANCE_ROOM = 219,
    xDC_THIEVES_TOWN_SOUTH_EAST_ENTRANCE_ROOM = 220,
    xDD_EMPTY_CLONE_ROOM24 = 221,
    xDE_ICE_PALACE_KHOLDSTARE_BOSS = 222,
    xDF__TOP_OF_BACKWARD_DEATH_MOUNTAIN_CAVE = 223,
    xE0_AGAHNIM_S_TOWER_ENTRANCE_ROOM = 224,
    xE1_CAVE_LOST_WOODS_HP = 225,
    xE2_CAVE_LUMBERJACK_S_TREE_HP = 226,
    xE3_CAVE_1_2_MAGIC = 227,
    xE4_CAVE_LOST_OLD_MAN_HOUSE_CAVE = 228,
    xE5_CAVE_LOST_OLD_MAN_HOUSE_CAVE_BACK = 229,
    xE6_CAVE_WITH_A_BUNCH_OF_KEESE = 230,
    xE7_CAVE_WITH_A_BUNCH_OF_KEESE_2 = 231,
    xE8_SUPER_BUNNY_CAVE_EXIT = 232,
    xE9_EMPTY_CLONE_ROOM25 = 233,
    xEA_CAVE_INSIDE_SPECTACLE_ROCK_HP = 234,
    xEB_BUMPER_CAVE_ENTRANCE1 = 235,
    xEC_EMPTY_CLONE_ROOM26 = 236,
    xED_CAVE_NO_CLUE = 237,
    xEE_CAVE_SPIRAL_CAVE = 238,
    xEF_CAVE_CRYSTAL_SWITCH_5_CHESTS_ROOM = 239,
    xF0_CAVE_LOST_OLD_MAN_STARTING_CAVE_ENTRANCE = 240,
    xF1_CAVE_LOST_OLD_MAN_STARTING_CAVE_EXIT = 241,
    xF2_HOUSE_OLD_WOMAN_NEXT_DOOR = 242,
    xF3_HOUSE_OLD_WOMAN_SAHASRAHLA_S_WIFE = 243,
    xF4_HOUSE_ANGRY_BROTHERS_EXIT_TO_MAZE_GAME = 244,
    xF5_HOUSE_ANGRY_BROTHERS_ENTRANCE = 245,
    xF6_EMPTY_CLONE_ROOM27 = 246,
    xF7_EMPTY_CLONE_ROOM28 = 247,
    xF8_SUPER_BUNNY_CAVE_ENTRANCE_AND_CHESTS = 248,
    xF9_SPECTICAL_ROCK_CAVE_EXIT_AFTER_FALLING_FROM_TOP_ENTRANCE = 249,
    xFA_SPECTICAL_ROCK_CAVE_ENTRANCE_AFTER_JUMPING_TO_GET_TO_HP = 250,
    xFB_BUMPER_CAVE_ENTRANCE2 = 251,
    xFC_EMPTY_CLONE_ROOM29 = 252,
    xFD_SOME_CAVE_ON_DEATH_MOUNTAIN = 253,
    xFE_SPIRAL_CAVE_EXIT_AFTER_FALLING = 254,
    xFF_UPSIDE_DOWN_CAVE_MIDDLE_ENTRANCE = 255,

    // Note: stairs between rooms probably CAN'T work for these rooms since
    // they exceed a byte.
    x100_SHOP_IN_LOST_WOODS_0X100 = 256,
    x101_SCARED_LADIES_HOUSES = 257,
    x102_SICK_KID = 258,
    x103_INN_BUSH_HOUSE = 259,
    x104_LINK_S_HOUSE = 260,
    x105_SAHASRAHLA_S_HOUSE = 261,
    x106_CHEST_GAME_OUTCAST_VILLAGE_BOMB_HOUSE = 262,
    x107_LIBRARY_BOMB_FARM_ROOM = 263,
    x108_CHICKEN_HOUSE = 264,
    x109_WITCH_HUT = 265,
    x10A_AGINAH_S_CAVE = 266,
    x10B_SWAMP_FLOODWAY_ROOM = 267,
    x10C_MIMIC_CAVE = 268,
    x10D_CAVE_OUTSIDE_MISERY_MIRE = 269,
    x10E_CAVE_0X10E_2_UNKNOWN_CAVES = 270,
    x10F_SHOP_0X10F = 271,
    x110_SHOP_0X110 = 272,
    x111_ARCHER_GAME = 273,
    x112_CAVE_SHOP_0X112 = 274,
    x113_KING_S_TOMB = 275,
    x114_WISHING_WELL_CAVE_0X114 = 276,
    x115_WISHING_WELL_BIG_FAIRY = 277,
    x116_GREATEST_FAIRY = 278,
    x117_SPIKE_CAVE = 279,
    x118_SHOP_0X118 = 280,
    x119_BLIND_S_HOUSE = 281,
    x11A_MUTANT_HUT = 282,
    x11B_MIRROR_CAVES_SOUTH_OF_TREE_BOY_ABOVE_KINGS_TOMB = 283,
    x11C_BOMB_SHOP = 284,
    x11D_BLIND_S_BASEMENT = 285,
    x11E_HYPE_CAVE = 286,
    x11F_SHOP_0X11F = 287,
    x120_ICE_ROD_CAVE = 288,
    x121_SMITHS_HOUSE = 289,
    x122_FORTUNE_TELLER_S = 290,
    x123_MINI_MOLDORM_CAVE = 291,
    x124_UNKNOWN_CAVE_BONK_CAVE = 292,
    x125_CAVE_0X125 = 293,
    x126_CHECKER_BOARD_CAVE = 294,
    x127_HAMMER_PEG_CAVE = 295,
}

impl Into<u16> for UWRoomId {
    fn into(self) -> u16 {
        self as u16
    }
}
