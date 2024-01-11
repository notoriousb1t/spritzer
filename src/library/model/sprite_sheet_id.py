from enum import IntEnum
from typing import Dict, List
from library.model.sprite_id import SpriteId


class SpriteSheetId(IntEnum):
    x0_FREESPACE = 0
    xC_OCTOROK_ZORA = 12
    xD_SOLDIERS_DW = 13
    xE_POE_THIEF = 14
    xF_DASH_HOARDER = 15
    x10_MISC_ROCKS = 16
    x11_MISC_FAKE_SWORD = 17
    x12_DESERT_1 = 18
    x13_SOLDIER_RECRUITS = 19
    x14_FRIENDLY_LYNEL = 20
    x15_THIEF_DW = 21
    x16_HINOX_SNAPDRAGON = 22
    x17_MOBLIN = 23
    x18_OCTOROCKS = 24
    x19_SWAMOLA_CROW = 25
    x1A_AGAHNIM = 26
    x1B_MISCELLANEOUS_DW_1 = 27
    x1C_PESTS = 28
    x1D_ARMOS_BOSS_LOCK_BAT = 29
    x1E_MINI_MONSTERS = 30
    x1F_STALFOS_BARI = 31
    x20_STALFOS_KNIGHT_VERMIN = 32
    x21_BIG_BAD_GUY = 33
    x22_WATER_TEKTITES = 34
    x23_WALLMASTER_GIBDO = 35
    x24_PESTS_DW = 36
    x25_WIZZROBE = 37
    x26_FROSTY_FRIENDS = 38
    x27_TURTLE_ROCK = 39
    x28_ZAZAK = 40
    x29_WIZZROBE = 41
    x2A_HAZARDS = 42
    x2B_UNUSED = 43
    x2C_BEAM_ME_UP_MR_POPO = 44
    x2D_UNKNOWN = 45
    x2E_EYEGORE = 46
    x2F_CANON_SANDCRAB = 47
    x30_MOLDORM_BOSS = 48
    x31_LANMOLAS_BOSS = 49
    x33_BIG_BAD_GUY = 51
    x36_MASTER_SWORD = 54
    x37_MASTER_SWORD = 55
    x38_MOTHULA_BOSS = 56
    x39_ARRGHUS_BOSS = 57
    x3A_HELMASAUR_KING_BOSS = 58
    x3B_BLIND_BOSS = 59
    x3C_KHOLDSTARE_BOSS = 60
    x3D_VITREOUS_BOSS = 61
    x3E_HELMASAUR_KING_BOSS = 62
    x3F_TRINEXX_BOSS = 63
    x40_TRINEXX_BOSS = 64
    x41_BIG_BAD_GUY = 65
    x42_AGAHNIM = 66
    x43_AGAHNIM = 67
    x44_ZORAS_DOMAIN = 68
    x45_BIG_BAD_GUY = 69
    x46_SOLDIERS = 70
    x47_PRIEST = 71
    x48_SOLDIER = 72
    x49_SOLDIERS = 73
    x4A_KAKARIKO = 74
    x4B_ARCHERY = 75
    x4C_SAHASRAHLA_WITCH = 76
    x4D_OLD_MAN_MAIDEN = 77
    x4E_UNCLE_PRIEST = 78
    x4F_OLD_MAN_RUNNER = 79
    x50_CUCCO_FOR_NPCS = 80
    x51_UNCLE_PRIEST_SICK_BOY = 81
    x52_ANTIFAIRY_SPIKES = 82
    x53_UNKNOWN = 83
    x55_AGAHNIM = 85
    x59_FOLLOWERS = 89
    x5A_UNKNOWN = 90
    x5C_UNKNOWN = 92
    x5D_MANTLE_CREDITS = 93

    def __str__(self) -> str:
        return self.name


def get_common_sprites() -> List[SpriteId]:
    """Sprites that are available for any spritesheet"""
    return [
        SpriteId.x33_RUPEE_PULL,
        SpriteId.x9F_HAUNTED_GROVE_RABBIT,
        SpriteId.xAC_APPLE,
        SpriteId.xBA_WHIRLPOOL,
        SpriteId.xD8_HEART,
        SpriteId.xD9_GREEN_RUPEE,
        SpriteId.xDA_BLUE_RUPEE,
        SpriteId.xDB_RED_RUPEE,
        SpriteId.x79_BEE,
        SpriteId.xB2_GOOD_BEE,
        SpriteId.xDC_BOMB_REFILL_1,
        SpriteId.xDD_BOMB_REFILL_4,
        SpriteId.xDE_BOMB_REFILL_8,
        SpriteId.xDF_SMALL_MAGIC_REFILL,
        SpriteId.xE0_FULL_MAGIC_REFILL,
        SpriteId.xE1_ARROW_REFILL_5,
        SpriteId.xE2_ARROW_REFILL_10,
        SpriteId.xE3_FAIRY,
        SpriteId.xE7_MUSHROOM,
        SpriteId.xEA_HEART_CONTAINER,
        SpriteId.xEB_HEART_PIECE,
        SpriteId.xF3_PERSONS_DOOR_OW_OVERLORD,
        SpriteId.x37_WATERFALL,  # TODO: needs verification
        SpriteId.x3B_BONK_ITEM,  # TODO: needs verification
        SpriteId.xD1_BUNNY_BEAM,  # TODO: Needs verification
        SpriteId.x76_ZELDA,
        SpriteId.xBA_WHIRLPOOL,
    ]


def common_underworld_sprites() -> List[SpriteId]:
    """Sprites that can be used regardless of the spritesheet"""
    return [
        SpriteId.xC5_MEDUSA,
        SpriteId.xC6_MEDUSA_FOUR_WAY,
        SpriteId.xE4_SMALL_KEY,
        SpriteId.xE5_BIG_KEY,
        SpriteId.x106_BOMB_DROP1_TRAP,
        SpriteId.x107_MOVING_FLOOR,
        SpriteId.x108_TRANSFORMER_BUNNY_BEAM,
        SpriteId.x116_ZORO_SPAWNER,
        SpriteId.x109_WALLMASTER_OVERLORD,
        SpriteId.x114_FLYING_FLOOR_TILE_TRAP,
        SpriteId.x10B_FLOOR_DROP_NORTH,
        SpriteId.x10A_FLOOR_DROP_SQUARE,
        SpriteId.x117_FOUR_SKULL_TRAP_IN_POD_UNDER_POT,
    ]


def create_spriteset_dict() -> Dict[SpriteSheetId, List[SpriteId]]:
    """Creates a new dictionary for existing spriteset -> sprite relationships. This should only contain information normally in vanilla"""
    spriteset_dict: Dict[SpriteSheetId, List[SpriteId]] = {
        SpriteSheetId.x0_FREESPACE: [],  # This should always be empty. This is effectively None.
        SpriteSheetId.xC_OCTOROK_ZORA: [
            SpriteId.x58_CRAB,
            SpriteId.x55_FIREBALL_ZORA,
            SpriteId.x8_OCTOROK,
            SpriteId.xA_OCTOROK_FOUR_WAY,
            SpriteId.xF_OCTOBALLOON,
            SpriteId.xD2_FLOPPING_FISH,
        ],
        SpriteSheetId.xD_SOLDIERS_DW: [
            SpriteId.x41_BLUE_GUARD,
            SpriteId.x42_GREEN_GUARD,
            SpriteId.x43_RED_SPEAR_GUARD,
            SpriteId.x45_GREEN_ASSAULT_GUARD,
        ],
        SpriteSheetId.xE_POE_THIEF: [
            SpriteId.x19_POE,
        ],
        SpriteSheetId.xF_DASH_HOARDER: [
            SpriteId.x17_HOARDER,
            SpriteId.x3B_BONK_ITEM,
            SpriteId.x3E_HOARDER_ROCK,
            # Creates a "Flying Hoarder"
            SpriteId.x19_POE,
        ],
        SpriteSheetId.x10_MISC_ROCKS: [
            SpriteId.x17_HOARDER,
            SpriteId.x27_DEADROCK,
            SpriteId.x3E_HOARDER_ROCK,
            SpriteId.x51_ARMOS_STATUE,
            SpriteId.xC2_BOULDER,
            SpriteId.xC9_TEKTITE,
            SpriteId.xF4_FALLING_ROCKS_OW_OVERLORD,
            # Creates a "Flying Hoarder"
            SpriteId.x19_POE,
        ],
        SpriteSheetId.x11_MISC_FAKE_SWORD: [
            SpriteId.x0_RAVEN,
            SpriteId.x17_HOARDER,
            SpriteId.x39_LOCKSMITH,
            SpriteId.x3E_HOARDER_ROCK,
            SpriteId.x4D_TOPPO,
            SpriteId.xD_BUZZBLOB,
            SpriteId.xE8_FAKE_MASTER_SWORD,
            # Creates a "Flying Hoarder"
            SpriteId.x19_POE,
        ],
        SpriteSheetId.x12_DESERT_1: [
            SpriteId.x1_VULTURE,
            SpriteId.x4C_GELDMAN,
            SpriteId.x57_DESERT_STATUE,
            SpriteId.xF2_MEDALLION_TABLET,
            SpriteId.xB3_PEDESTAL_PLAQUE,
            SpriteId.x0_RAVEN,
            SpriteId.x39_LOCKSMITH,
        ],
        SpriteSheetId.x13_SOLDIER_RECRUITS: [
            SpriteId.x4B_GREEN_KNIFE_GUARD,
        ],
        SpriteSheetId.x14_FRIENDLY_LYNEL: [
            SpriteId.xB9_BULLY_AND_FRIEND,
            SpriteId.xD0_LYNEL,
        ],
        SpriteSheetId.x15_THIEF_DW: [
            SpriteId.xB4_PURPLE_CHEST,
            SpriteId.xBB_SALESMAN,
            SpriteId.xB_CUCCO,
            SpriteId.x14_THIEVES_TOWN_GRATE,
            SpriteId.x19_POE,
        ],
        SpriteSheetId.x16_HINOX_SNAPDRAGON: [
            SpriteId.xE_SNAPDRAGON,
            SpriteId.x11_HINOX,
            SpriteId.x22_ROPA,
        ],
        SpriteSheetId.x17_MOBLIN: [
            SpriteId.x12_MOBLIN,
            SpriteId.xE_SNAPDRAGON,
        ],
        SpriteSheetId.x18_OCTOROCKS: [
            SpriteId.x55_FIREBALL_ZORA,
            SpriteId.x8_OCTOROK,
            SpriteId.xA_OCTOROK_FOUR_WAY,
            SpriteId.xC0_CATFISH,
        ],
        SpriteSheetId.x19_SWAMOLA_CROW: [
            SpriteId.x0_RAVEN,
            SpriteId.x25_TALKING_TREE,
            SpriteId.xB6_KIKI,
            SpriteId.xCF_SWAMOLA,
        ],
        SpriteSheetId.x1A_AGAHNIM: [
            SpriteId.x7A_AGAHNIM,
            SpriteId.xC1_AGAHNIM_TELEPORTING,
            SpriteId.x7B_AGAHNIM_ENERGY_BALL,
            SpriteId.xAB_CRYSTAL_MAIDEN,
            SpriteId.x76_ZELDA,
        ],
        SpriteSheetId.x1B_MISCELLANEOUS_DW_1: [
            SpriteId.x25_TALKING_TREE,
            SpriteId.xA8_GREEN_ZIRRO,
            SpriteId.xA9_BLUE_ZIRRO,
            SpriteId.xAA_PIKIT_LIKE_LIKE,
            SpriteId.x19_POE,
        ],
        SpriteSheetId.x1C_PESTS: [
            SpriteId.x6D_RAT_CRICKET,
            SpriteId.x6E_ROPE,
            SpriteId.x6F_KEESE,
            SpriteId.x40_LIGHTNING_LOCK,
        ],
        SpriteSheetId.x1D_ARMOS_BOSS_LOCK_BAT: [
            SpriteId.x119_ARMOS_KNIGHTS_TRIGGER,
            SpriteId.x3A_MAGIC_BAT,
            SpriteId.x40_LIGHTNING_LOCK,
            SpriteId.x53_ARMOS_KNIGHT,
            SpriteId.x30_RACE_GUY,
        ],
        SpriteSheetId.x1E_MINI_MONSTERS: [
            SpriteId.x13_MINI_HELMASAUR,
            SpriteId.x18_MINI_MOLDORM,
            SpriteId.x26_HARDHAT_BEETLE,
        ],
        SpriteSheetId.x1F_STALFOS_BARI: [
            SpriteId.x24_BLUE_BARI,
            SpriteId.x23_RED_BARI,
            SpriteId.xA7_STALFOS,
            SpriteId.x7C_GREEN_STALFOS,
        ],
        SpriteSheetId.x20_STALFOS_KNIGHT_VERMIN: [
            SpriteId.x110_PIROGUSU_SPAWNER_RIGHT,
            SpriteId.x111_PIROGUSU_SPAWNER_LEFT,
            SpriteId.x112_PIROGUSU_SPAWNER_DOWN,
            SpriteId.x113_PIROGUSU_SPAWNER_UP,
            SpriteId.x8F_BLOB,
            SpriteId.x91_STALFOS_KNIGHT,
            SpriteId.x9C_BABASU_SOUTH,
            SpriteId.x9D_BABUSU_EAST,
        ],
        SpriteSheetId.x21_BIG_BAD_GUY: [
            SpriteId.xD6_GANON,
            SpriteId.xD7_GANON_INVINCIBLE,
        ],
        SpriteSheetId.x22_WATER_TEKTITES: [
            SpriteId.x9A_KYAMERON,
            SpriteId.x81_WATER_TEKTITE,
        ],
        SpriteSheetId.x23_WALLMASTER_GIBDO: [
            SpriteId.x8B_GIBDO,
            SpriteId.x90_WALLMASTER,
        ],
        SpriteSheetId.x24_PESTS_DW: [
            SpriteId.x6F_KEESE,
            SpriteId.x6D_RAT_CRICKET,
            SpriteId.x6E_ROPE,
        ],
        SpriteSheetId.x25_WIZZROBE: [
            SpriteId.x115_WIZZROBE_SPAWNER,
            SpriteId.x20_SLUGGULA,
            SpriteId.x9B_WIZZROBE,
        ],
        SpriteSheetId.x26_FROSTY_FRIENDS: [
            SpriteId.x99_PENGATOR,
            SpriteId.xA1_FREEZOR,
        ],
        SpriteSheetId.x27_TURTLE_ROCK: [
            SpriteId.x1E_CRYSTAL_SWITCH,
            SpriteId.x5D_ROLLER_SOUTH,
            SpriteId.x5E_ROLLER_NORTH,
            SpriteId.x5F_ROLLER_EAST,
            SpriteId.x60_ROLLER_WEST,
            SpriteId.xC7_POKEY,
            SpriteId.xCA_CHAIN_CHOMP,
            SpriteId.xED_SOMARIA_PLATFORM,
        ],
        SpriteSheetId.x28_ZAZAK: [
            SpriteId.xA5_BLUE_ZAZAK,
            SpriteId.xA6_RED_ZAZAK,
            SpriteId.xC3_GIBO,
        ],
        SpriteSheetId.x29_WIZZROBE: [
            SpriteId.x115_WIZZROBE_SPAWNER,
            SpriteId.x9B_WIZZROBE,
        ],
        SpriteSheetId.x2A_HAZARDS: [
            SpriteId.x7E_FIREBAR_CLOCKWISE,
            SpriteId.x7F_FIREBAR_COUNTER_CLOCKWISE,
            SpriteId.x80_FIRESNAKE,
            SpriteId.x87_KODONGO_FIRE,
            SpriteId.x8E_TERRORPIN,
            SpriteId.xD5_DIGGING_GAME_PROPRIETOR,
            SpriteId.x86_KODONGO,
        ],
        SpriteSheetId.x2B_UNUSED: [],  # 80=0x50
        SpriteSheetId.x2C_BEAM_ME_UP_MR_POPO: [
            SpriteId.x4E_POPO_1,
            SpriteId.x4F_POPO_2,
            SpriteId.x61_BEAMOS,
            SpriteId.x5C_SPARK_RIGHT_TO_LEFT,
            SpriteId.x83_GREEN_EYEGORE_MIMIC,
            SpriteId.x84_RED_EYEGORE_MIMIC,
        ],
        SpriteSheetId.x2E_EYEGORE: [
            SpriteId.x83_GREEN_EYEGORE_MIMIC,
            SpriteId.x84_RED_EYEGORE_MIMIC,
        ],
        SpriteSheetId.x2F_CANON_SANDCRAB: [
            SpriteId.x63_DEVALANT_PIT,
            SpriteId.x64_DEVALANT,
            SpriteId.x66_MOVING_CANNON_WEST,
            SpriteId.x67_MOVING_CANNON_EAST,
            SpriteId.x68_MOVING_CANNON_SOUTH,
            SpriteId.x69_MOVING_CANNON_NORTH,
            SpriteId.x71_LEEVER,
        ],
        SpriteSheetId.x30_MOLDORM_BOSS: [
            SpriteId.x9_MOLDORM,
        ],
        SpriteSheetId.x31_LANMOLAS_BOSS: [
            SpriteId.x54_LANMOLAS,
        ],
        SpriteSheetId.x33_BIG_BAD_GUY: [
            SpriteId.xD6_GANON,
            SpriteId.xD7_GANON_INVINCIBLE,
        ],
        SpriteSheetId.x36_MASTER_SWORD: [
            SpriteId.x59_LOST_WOODS_BIRD,
            SpriteId.x5A_LOST_WOODS_SQUIRREL,
            SpriteId.x62_MASTERSWORD,
            SpriteId.x72_FAIRY_POND_TRIGGER,
        ],
        SpriteSheetId.x37_MASTER_SWORD: [
            SpriteId.x2B_TENTMAN,
            SpriteId.x62_MASTERSWORD,  # Requires multiple Spritesets.
        ],
        SpriteSheetId.x38_MOTHULA_BOSS: [
            SpriteId.x88_MOTHULA,
            SpriteId.x89_MOTHULA_BEAM,
        ],
        SpriteSheetId.x39_ARRGHUS_BOSS: [
            SpriteId.x8C_ARRGHUS,
            SpriteId.x8D_ARRGHUS_SPAWN,
            SpriteId.xC8_GREAT_FAIRY,
        ],
        SpriteSheetId.x3A_HELMASAUR_KING_BOSS: [
            SpriteId.x70_HELMASAUR_KING_FIREBALL,
            SpriteId.x92_KING_HELMASAUR,
        ],
        SpriteSheetId.x3B_BLIND_BOSS: [SpriteId.xCE_BLIND],
        SpriteSheetId.x3C_KHOLDSTARE_BOSS: [
            SpriteId.xA2_KHOLDSTARE,
            SpriteId.xA3_KHOLDSTARES_SHELL,
            SpriteId.xA4_FALLING_ICE,
        ],
        SpriteSheetId.x3D_VITREOUS_BOSS: [
            SpriteId.xBF_LIGHTNING,
            SpriteId.xBD_VITREOUS,
            SpriteId.xBE_VITREOUS_SMALL_EYEBALL,
        ],
        SpriteSheetId.x3E_HELMASAUR_KING_BOSS: [
            SpriteId.x92_KING_HELMASAUR,
            SpriteId.x70_HELMASAUR_KING_FIREBALL,
        ],  # 85=0x55
        SpriteSheetId.x3F_TRINEXX_BOSS: [
            SpriteId.xCB_TRINEXX_ROCK,
            SpriteId.xCC_TRINEXX_FIRE,
            SpriteId.xCD_TRINEXX_ICE,
        ],
        SpriteSheetId.x40_TRINEXX_BOSS: [
            SpriteId.xCB_TRINEXX_ROCK,
            SpriteId.xCC_TRINEXX_FIRE,
            SpriteId.xCD_TRINEXX_ICE,
        ],
        SpriteSheetId.x41_BIG_BAD_GUY: [
            SpriteId.xD6_GANON,
            SpriteId.xD7_GANON_INVINCIBLE,
        ],  # 98=0x62
        SpriteSheetId.x42_AGAHNIM: [
            SpriteId.x7A_AGAHNIM,
            SpriteId.xC1_AGAHNIM_TELEPORTING,
            SpriteId.x7B_AGAHNIM_ENERGY_BALL,
            SpriteId.xAB_CRYSTAL_MAIDEN,
            SpriteId.x76_ZELDA,
        ],  # 82=0x52  88=0x58
        SpriteSheetId.x43_AGAHNIM: [
            SpriteId.x7A_AGAHNIM,
            SpriteId.xC1_AGAHNIM_TELEPORTING,
            SpriteId.x7B_AGAHNIM_ENERGY_BALL,
            SpriteId.xAB_CRYSTAL_MAIDEN,
            SpriteId.x76_ZELDA,
        ],  # 82=0x52  88=0x58
        SpriteSheetId.x44_ZORAS_DOMAIN: [
            SpriteId.x52_ZORA_KING,
            SpriteId.x56_WALKING_ZORA,
        ],  # 14=0xe
        SpriteSheetId.x45_BIG_BAD_GUY: [
            SpriteId.xD6_GANON,
            SpriteId.xD7_GANON_INVINCIBLE,
        ],  # 98=0x62
        SpriteSheetId.x46_SOLDIERS: [
            SpriteId.x6B_CANNON_GUARD,
            SpriteId.x47_GREEN_GUARD_BUSH,
            SpriteId.x6A_BALL_N_CHAIN_GUARD,
            SpriteId.x48_RED_JAVELIN_GUARD,
            SpriteId.x44_BLUE_ASSAULT_GUARD,
            SpriteId.x4_PULL_SWITCH_NORMAL,
            SpriteId.x6_PULL_SWITCH_TRAP,
        ],
        SpriteSheetId.x47_PRIEST: [
            SpriteId.x73_UNCLE_PRIEST_MANTLE,
        ],  # 70=0x46
        SpriteSheetId.x48_SOLDIER: [
            SpriteId.x46_BLUE_ARCHER,
            SpriteId.x47_GREEN_GUARD_BUSH,
            SpriteId.x3F_TUTORIAL_SOLDIER,
            SpriteId.x49_RED_GUARD_BUSH,
        ],
        SpriteSheetId.x49_SOLDIERS: [
            SpriteId.x41_BLUE_GUARD,
            SpriteId.x42_GREEN_GUARD,
            SpriteId.x43_RED_SPEAR_GUARD,
            SpriteId.x45_GREEN_ASSAULT_GUARD,
            SpriteId.x43_RED_SPEAR_GUARD,
            SpriteId.xB9_BULLY_AND_FRIEND,
        ],
        SpriteSheetId.x4A_KAKARIKO: [
            SpriteId.x75_BOTTLE_SALESMAN,
            SpriteId.x3C_VILLAGE_KID,
            SpriteId.x2C_LUMBERJACKS,
            SpriteId.x2A_SWEEPING_LADY,
            SpriteId.x78_VILLAGE_ELDER,
            SpriteId.x1A_SMITHY,
            SpriteId.xBB_SALESMAN,
        ],
        SpriteSheetId.x4B_ARCHERY: [
            SpriteId.x65_ARCHERY_GUY,
            SpriteId.x31_FORTUNE_TELLER,
            SpriteId.x28_HINT_PC_DW,
            SpriteId.xE9_MAGIC_MERCHANT,
        ],
        SpriteSheetId.x4C_SAHASRAHLA_WITCH: [
            SpriteId.x16_SAHASRAHLA,
            SpriteId.x2E_FLUTE_KID,
            SpriteId.x36_WITCH,
        ],
        SpriteSheetId.x4D_OLD_MAN_MAIDEN: [
            SpriteId.xAB_CRYSTAL_MAIDEN,
            SpriteId.xAD_LOST_OLD_MAN,
            SpriteId.xB7_BLIND_MAIDEN,
            SpriteId.xB5_BOMB_SALESMAN,
        ],
        SpriteSheetId.x4E_UNCLE_PRIEST: [
            SpriteId.x73_UNCLE_PRIEST_MANTLE,
            SpriteId.xA0_HAUNTED_GROVE_BIRD,
            SpriteId.x2E_FLUTE_KID,
            SpriteId.x1D_FLUTEQUEST,
            SpriteId.x9E_HAUNTED_GROVE_OSTRICH,
        ],
        SpriteSheetId.x4F_OLD_MAN_RUNNER: [
            SpriteId.x32_ANGRY_BROTHERS,
            SpriteId.x74_RUNNING_MAN,
            SpriteId.xAD_LOST_OLD_MAN,
        ],
        SpriteSheetId.x50_CUCCO_FOR_NPCS: [
            SpriteId.xB_CUCCO,
            SpriteId.x2F_RACE_LADY,
            SpriteId.x30_RACE_GUY,
            SpriteId.x34_SNITCH_YOUNG,
            SpriteId.x3D_SNITCH_OLD,
        ],
        SpriteSheetId.x51_UNCLE_PRIEST_SICK_BOY: [
            SpriteId.x73_UNCLE_PRIEST_MANTLE,
            SpriteId.x1F_BUG_CATCHING_KID,
        ],  # 77=0x4d
        SpriteSheetId.x52_ANTIFAIRY_SPIKES: [
            SpriteId.x15_ANTIFAIRY,
            SpriteId.x77_ANTIFAIRY_2,
            SpriteId.x7D_BIG_SPIKE,
            SpriteId.x82_ANTIFAIRY_CIRCLE,
            SpriteId.x8A_SPIKE_BLOCK,
            SpriteId.x93_BUMPER,
            SpriteId.x95_EYE_LASER_EAST,
            SpriteId.x96_EYE_LASER_WEST,
            SpriteId.x97_EYE_LASER_SOUTH,
            SpriteId.x97_EYE_LASER_SOUTH,
            SpriteId.x1E_CRYSTAL_SWITCH,
            SpriteId.x1C_STATUE,
            SpriteId.x4_PULL_SWITCH_NORMAL,
            SpriteId.x6_PULL_SWITCH_TRAP,
        ],
        SpriteSheetId.x53_UNKNOWN: [
            SpriteId.x1E_CRYSTAL_SWITCH,
            SpriteId.x1C_STATUE,
            SpriteId.x4_PULL_SWITCH_NORMAL,
            SpriteId.x6_PULL_SWITCH_TRAP,
            SpriteId.x80_FIRESNAKE,
            SpriteId.x8A_SPIKE_BLOCK,
            SpriteId.x21_WATER_SWITCH,
            SpriteId.x5B_SPARK_LEFT_TO_RIGHT,
            SpriteId.x5C_SPARK_RIGHT_TO_LEFT,
            SpriteId.x7E_FIREBAR_CLOCKWISE,
            SpriteId.x7F_FIREBAR_COUNTER_CLOCKWISE,
            SpriteId.x96_EYE_LASER_WEST,
            SpriteId.x97_EYE_LASER_SOUTH,
            SpriteId.x15_ANTIFAIRY,
            SpriteId.x77_ANTIFAIRY_2,
        ],
        SpriteSheetId.x55_AGAHNIM: [
            SpriteId.x7A_AGAHNIM,
            SpriteId.xC1_AGAHNIM_TELEPORTING,
            SpriteId.x7B_AGAHNIM_ENERGY_BALL,
            SpriteId.xAB_CRYSTAL_MAIDEN,
            SpriteId.x76_ZELDA,
            SpriteId.x4_PULL_SWITCH_NORMAL,
            SpriteId.x6_PULL_SWITCH_TRAP,
        ],
        SpriteSheetId.x59_FOLLOWERS: [
            SpriteId.xB6_KIKI,
        ],
        SpriteSheetId.x5A_UNKNOWN: [],
        SpriteSheetId.x5C_UNKNOWN: [],
        SpriteSheetId.x5D_MANTLE_CREDITS: [
            SpriteId.xEE_CASTLE_MANTLE
        ],  # Contains intro/outro sprites as well, but they are not listed.
    }

    return spriteset_dict
