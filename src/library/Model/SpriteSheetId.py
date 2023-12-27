from enum import IntEnum
from typing import Dict

from .SpriteId import SpriteId


class SpriteSheetId(IntEnum):
    x0_FREESPACE = 0
    xC_OCTOROK_ZORA = 12  # 1=0x1  4=0x4  5=0x5  11=0xb  14=0xe  32=0x20  35=0x23
    xD_SOLDIERS_DW = 13  # 18=0x12  19=0x13  20=0x14  21=0x15  22=0x16  23=0x17  24=0x18  25=0x19  30=0x1e  34=0x22  36=0x24  37=0x25  100=0x64
    xE_POE_THIEF = 14  # 3=0x3  7=0x7  37=0x25  104=0x68
    xF_DASH_HOARDER = 15  # 16=0x10  26=0x1a
    x10_MISCELLANEOUS_1 = 16  # 5=0x5  16=0x10  30=0x1e  33=0x21
    x11_MISCELLANEOUS_2 = (
        17  # 4=0x4  7=0x7  10=0xa  13=0xd  26=0x1a  32=0x20  34=0x22  36=0x24  37=0x25
    )
    x12_DESERT_1 = 18  # 8=0x8  16=0x10  31=0x1f  33=0x21
    x13_SOLDIER_RECRUITS = 19  # 2=0x2  3=0x3  68=0x44  77=0x4d  78=0x4e  103=0x67
    x14_FRIENDLY_LYNEL = 20  # 20=0x14
    x15_THIEF_DW = 21  # 21=0x15  27=0x1b  29=0x1d  106=0x6a
    x16_HINOX_SNAPDRAGON = 22  # 19=0x13  20=0x14  22=0x16  23=0x17  24=0x18  25=0x19  28=0x1c  30=0x1e  31=0x1f  32=0x20  34=0x22  35=0x23  36=0x24  38=0x26
    x17_MOBLIN = (
        23  # 18=0x12  19=0x13  20=0x14  21=0x15  23=0x17  28=0x1c  30=0x1e  38=0x26
    )
    x18_OCTOROCKS = 24  # 22=0x16  25=0x19
    x19_SWAMOLA_CROW = 25  # 22=0x16  23=0x17
    x1A_AGAHNIM = 26  # 38=0x26  88=0x58
    x1B_MISCELLANEOUS_DW_1 = 27  # 19=0x13  25=0x19  38=0x26
    x1C_PESTS = 28  # 65=0x41  66=0x42  70=0x46
    x1D_ARMOS_BOSS_LOCK_BAT = 29  # 2=0x2  73=0x49
    x1E_MINI_MONSTERS = 30  # 76=0x4c  83=0x53  84=0x54  85=0x55  86=0x56  87=0x57  89=0x59  90=0x5a  101=0x65  104=0x68
    x1F_STALFOS_BARI = 31
    x20_STALFOS_KNIGHT_VERMIN = 32
    x21_BIG_BAD_GUY = 33  # 98=0x62
    x22_WATER_TEKTITES = 34  # 81=0x51
    x23_WALLMASTER_GIBDO = 35  # 83=0x53
    x24_PESTS_DW = 36  # 97=0x61
    x25_WIZZROBE = 37  # 93=0x5d
    x26_FROSTY_FRIENDS = 38  # 92=0x5c  105=0x69
    x27_TURTLE_ROCK = 39  # 87=0x57  94=0x5e  101=0x65  102=0x66
    x28_ZAZAK = 40  # 91=0x5b  99=0x63
    x29_WIZZROBE = 41  # 100=0x64
    x2A_HAZARDS = 42  # 27=0x1b  89=0x59  107=0x6b
    x2B_UNUSED = 43  # 80=0x50
    x2C_BEAM_ME_UP_MR_POPO = 44
    x2E_EYEGORE = 46  # 72=0x48  73=0x49  74=0x4a  75=0x4b
    x2F_CANON_CRAB = 47  # 74=0x4a  75=0x4b
    x30_MOLDORM_BOSS = 48  # 76=0x4c
    x31_LANMOLAS_BOSS = 49  # 75=0x4b  99=0x63
    x33_BIG_BAD_GUY = 51  # 98=0x62
    x36_MASTER_SWORD = 54  # 12=0xc  71=0x47
    x37_MASTER_SWORD = 55  # 12=0xc
    x38_MOTHULA_BOSS = 56  # 90=0x5a
    x39_ARRGHUS_BOSS = 57  # 71=0x47  84=0x54
    x3A_HELMASAUR_KING_BOSS = 58  # 84=0x54  85=0x55
    x3B_BLIND_BOSS = 59  # 96=0x60
    x3C_KHOLDSTARE_BOSS = 60  # 86=0x56
    x3D_VITREOUS_BOSS = 61  # 82=0x52  86=0x56
    x3E_HELMASAUR_KING_BOSS = 62  # 85=0x55
    x3F_TRINEXX_BOSS = 63  # 1=0x1  28=0x1c  87=0x57
    x40_TRINEXX_BOSS = 64  # 87=0x57
    x41_BIG_BAD_GUY = 65  # 98=0x62
    x42_AGAHNIM = 66  # 82=0x52  88=0x58
    x43_AGAHNIM = 67  # 82=0x52  88=0x58
    x44_ZORAS_DOMAIN = 68  # 14=0xe
    x45_BIG_BAD_GUY = 69  # 98=0x62
    x46_SOLDIERS_1 = 70
    x47_PRIEST = 71
    x48_SOLDIER = 72  # 2=0x2  4=0x4  5=0x5  11=0xb  13=0xd  103=0x67
    x49_SOLDIERS = 73  # 0=0x0  1=0x1  2=0x2  3=0x3  4=0x4  5=0x5  6=0x6  7=0x7  8=0x8  9=0x9  10=0xa  11=0xb  13=0xd  26=0x1a  28=0x1c  31=0x1f  32=0x20  35=0x23  65=0x41  66=0x42  67=0x43  68=0x44  70=0x46  77=0x4d  78=0x4e  80=0x50  97=0x61  103=0x67
    x4A_KAKARIKO = 74  # 6=0x6  7=0x7  26=0x1a  37=0x25  69=0x45  79=0x4f  104=0x68
    x4B_ARCHERY = 75  # 27=0x1b  69=0x45  71=0x47  80=0x50
    x4C_SAHASRAHLA_WITCH = 76  # 13=0xd  17=0x11  36=0x24  80=0x50
    x4D_OLD_MAN_MAIDEN = 77  # 69=0x45  71=0x47  79=0x4f
    x4E_UNCLE_PRIEST = 78  # 15=0xf
    x4F_OLD_MAN_RUNNER = 79  # 6=0x6  78=0x4e  79=0x4f
    x50_CUCCO_FOR_NPCS = 80  # 6=0x6  9=0x9  78=0x4e  79=0x4f  104=0x68
    x51_UNCLE_PRIEST_SICK_BOY = 81  # 77=0x4d
    x52_ANTIFAIRY_SPIKES = 82  # 65=0x41  66=0x42  67=0x43  68=0x44  70=0x46  72=0x48  74=0x4a  76=0x4c  83=0x53  89=0x59  90=0x5a  91=0x5b  92=0x5c  93=0x5d  94=0x5e  96=0x60  97=0x61  100=0x64  101=0x65  103=0x67  107=0x6b
    x53_UNKNOWN = 83  # 81=0x51  102=0x66  105=0x69
    x55_AGAHNIM = 85  # 82=0x52  88=0x58
    x5A_UNKNOWN = 90  # 69=0x45
    x5C_UNKNOWN = 92  # 27=0x1b
    x5D_MANTLE_CREDITS = 93  # 14=0xe  67=0x43

    def __str__(self) -> str:
        return self.name


def create_spriteset_dict() -> [Dict[SpriteSheetId, SpriteSheetId]]:
    """Creates a new dictionary for existing spriteset -> sprite relationships. This should only contain information normally in vanilla"""
    return {
        SpriteSheetId.x0_FREESPACE: [],  # This should always be empty. This is effectively None.
        SpriteSheetId.xC_OCTOROK_ZORA: [
            SpriteId.x55_FIREBALL_ZORA,
            SpriteId.x8_OCTOROK_ONE_WAY,
            SpriteId.xA_OCTOROK_FOUR_WAY,
            SpriteId.xF_OCTOBALLOON,
        ],
        SpriteSheetId.xD_SOLDIERS_DW: [
            SpriteId.x41_BLUE_SWORD_SOLDIER,
            SpriteId.x42_GREEN_SWORD_SOLDIER,
            SpriteId.x45_GREEN_SPEAR_SOLDIER,
        ],
        SpriteSheetId.xE_POE_THIEF: [
            SpriteId.x19_POE,
            SpriteId.xC4_THIEF,
        ],
        SpriteSheetId.xF_DASH_HOARDER: [
            SpriteId.x17_BUSH_HOARDER,
            SpriteId.x3B_DASH_ITEM,
            SpriteId.x3E_ROCK_HOARDER,
        ],
        SpriteSheetId.x10_MISCELLANEOUS_1: [
            SpriteId.x17_BUSH_HOARDER,
            SpriteId.x27_DEADROCK,
            SpriteId.x3E_ROCK_HOARDER,
            SpriteId.x51_ARMOS,
            SpriteId.xC2_BOULDERS,
            SpriteId.xC9_TEKTITE,
        ],
        SpriteSheetId.x11_MISCELLANEOUS_2: [
            SpriteId.x0_CROW,
            SpriteId.x17_BUSH_HOARDER,
            SpriteId.x39_AVERAGE_MIDDLE_AGED_MAN,
            SpriteId.x3E_ROCK_HOARDER,
            SpriteId.x4D_TOPPO,
            SpriteId.xD_BUZZBLOB,
            SpriteId.xE8_FAKE_MASTER_SWORD,
        ],
        SpriteSheetId.x12_DESERT_1: [
            SpriteId.x1_VULTURE,
            SpriteId.x4C_GELDMAN,
            SpriteId.x57_DESERT_PALACE_BARRIERS,
            SpriteId.xF2_MEDALLION_TABLET,
        ],
        SpriteSheetId.x13_SOLDIER_RECRUITS: [
            SpriteId.x4B_GREEN_SOLDIER_RECRUITS,
        ],
        SpriteSheetId.x14_FRIENDLY_LYNEL: [
            SpriteId.xB9_FEUDING_FRIENDS_ON_DEATH_MOUNTAIN,
            SpriteId.xD0_LYNEL,
        ],
        SpriteSheetId.x15_THIEF_DW: [
            SpriteId.xB4_THIEFS_CHEST_ITEM_TRIGGER,
        ],
        SpriteSheetId.x16_HINOX_SNAPDRAGON: [
            SpriteId.x11_HINOX,
            SpriteId.xE_SNAPDRAGON,
        ],
        SpriteSheetId.x17_MOBLIN: [
            SpriteId.x12_MOBLIN,
        ],
        SpriteSheetId.x18_OCTOROCKS: [
            SpriteId.x55_FIREBALL_ZORA,
            SpriteId.x8_OCTOROK_ONE_WAY,
            SpriteId.xA_OCTOROK_FOUR_WAY,
            SpriteId.xC0_CATFISH,
        ],
        SpriteSheetId.x19_SWAMOLA_CROW: [
            SpriteId.x0_CROW,
            SpriteId.xCF_SWAMOLA,
        ],
        SpriteSheetId.x1A_AGAHNIM: [
            SpriteId.x7A_AGAHNIM,
            SpriteId.xC1_AGAHNIM_TELEPORTING,
            SpriteId.x7B_AGAHNIM_ENERGY_BALL,
            SpriteId.xAB_MAIDEN,
            SpriteId.x76_PRINCESS_ZELDA,
        ],  # 38=0x26  88=0x58
        SpriteSheetId.x1B_MISCELLANEOUS_DW_1: [
            SpriteId.x25_TALKING_TREE,
            SpriteId.xA8_ZIRRO_1,
            SpriteId.xA9_ZIRRO_2,
            SpriteId.xAA_PIKIT,
        ],
        SpriteSheetId.x1C_PESTS: [
            SpriteId.x6D_RAT,
            SpriteId.x6E_ROPE,
            SpriteId.x6F_KEESE,
        ],  # 65=0x41  66=0x42  70=0x46
        SpriteSheetId.x1D_ARMOS_BOSS_LOCK_BAT: [
            SpriteId.x119_ARMOS_KNIGHTS_TRIGGER,
            SpriteId.x3A_HALF_MAGIC_BAT,
            SpriteId.x40_LIGHTNING_LOCK,
            SpriteId.x53_ARMOS_KNIGHTS_BOSS,
        ],
        SpriteSheetId.x1E_MINI_MONSTERS: [
            SpriteId.x13_MINI_HELMASAUR,
            SpriteId.x18_MINI_MOLDORM,
            SpriteId.x26_HARDHAT_BEETLE,
        ],  # 76=0x4c  83=0x53  84=0x54  85=0x55  86=0x56  87=0x57  89=0x59  90=0x5a  101=0x65  104=0x68
        SpriteSheetId.x1F_STALFOS_BARI: [
            SpriteId.x24_BLUE_BARI,
            SpriteId.x23_RED_BARI,
            SpriteId.xA7_STALFOS,
        ],
        SpriteSheetId.x20_STALFOS_KNIGHT_VERMIN: [
            SpriteId.x8F_SLIME,
            SpriteId.x91_STALFOS_KNIGHT,
            SpriteId.x9D_VERMIN_VERTICAL_PIROGUSU,
        ],
        SpriteSheetId.x21_BIG_BAD_GUY: [
            SpriteId.xD6_GANON,
            SpriteId.xD7_GANON_INVINCIBLE,
        ],  # 98=0x62
        SpriteSheetId.x22_WATER_TEKTITES: [
            SpriteId.x9A_KYAMERON_WATER_SPLASH,
            SpriteId.x81_WATER_TEKTITE,
        ],
        SpriteSheetId.x23_WALLMASTER_GIBDO: [
            SpriteId.x8B_GIBDO,
            SpriteId.x90_WALLMASTER,
        ],
        SpriteSheetId.x24_PESTS_DW: [
            SpriteId.x6F_KEESE,
            SpriteId.x6D_RAT,
            SpriteId.x6E_ROPE,
        ],
        SpriteSheetId.x25_WIZZROBE: [
            SpriteId.x115_WIZZROBE_SPAWNER,
            SpriteId.x20_SLUGGULA,
            SpriteId.x9B_WIZZROBE,
        ],  # 93=0x5d
        SpriteSheetId.x26_FROSTY_FRIENDS: [
            SpriteId.x99_PENGATOR,
            SpriteId.xA1_FREEZOR,
        ],  # 92=0x5c  105=0x69
        SpriteSheetId.x27_TURTLE_ROCK: [
            SpriteId.x1E_CRYSTAL_SWITCH,
            SpriteId.x5D_ROLLER_VERTICAL_MOVING_1,
            SpriteId.x5E_ROLLER_VERTICAL_MOVING_2,
            SpriteId.x5F_ROLLER,
            SpriteId.x60_ROLLER_HORIZONTAL_MOVING,
            SpriteId.xC7_HOKKU_BOKKU,
            SpriteId.xCA_CHAIN_CHOMP,
            SpriteId.xED_CANE_OF_SOMARIA_PLATFORM,
        ],
        SpriteSheetId.x28_ZAZAK: [
            SpriteId.xA5_BLUE_ZAZAK,
            SpriteId.xA6_RED_ZAZAK,
            SpriteId.xC3_GIBO,
        ],  # 91=0x5b  99=0x63
        SpriteSheetId.x29_WIZZROBE: [
            SpriteId.x115_WIZZROBE_SPAWNER,
            SpriteId.x9B_WIZZROBE,
        ],  # 100=0x64
        SpriteSheetId.x2A_HAZARDS: [
            SpriteId.x7E_GURUGURU_BAR_CLOCKWISE,
            SpriteId.x7F_GURUGURU_BAR_COUNTER_CLOCKWISE,
            SpriteId.x80_WINDER,
            SpriteId.x87_FLAMES,
            SpriteId.x8E_TERRORPIN,
            SpriteId.xD5_DIGGING_GAME_PROPRIETOR,
        ],  # 27=0x1b  89=0x59  107=0x6b
        SpriteSheetId.x2B_UNUSED: [],  # 80=0x50
        SpriteSheetId.x2C_BEAM_ME_UP_MR_POPO: [
            SpriteId.x4E_POPO_1,
            SpriteId.x4F_POPO_2,
            SpriteId.x61_BEAMOS,
            SpriteId.x83_GREEN_EYEGORE_MIMIC,
            SpriteId.x84_RED_EYEGORE_MIMIC,
        ],
        SpriteSheetId.x2E_EYEGORE: [
            SpriteId.x83_GREEN_EYEGORE_MIMIC,
            SpriteId.x84_RED_EYEGORE_MIMIC,
        ],  # 72=0x48  73=0x49  74=0x4a  75=0x4b
        SpriteSheetId.x2F_CANON_CRAB: [
            SpriteId.x58_CRAB,
            SpriteId.x66_MOVING_CANNON_BALL_SHOOTERS_RIGHT,
            SpriteId.x67_MOVING_CANNON_BALL_SHOOTERS_LEFT,
            SpriteId.x68_MOVING_CANNON_BALL_SHOOTERS_DOWN,
            SpriteId.x69_MOVING_CANNON_BALL_SHOOTERS_UP,
            SpriteId.x71_LEEVER,
        ],  # 74=0x4a  75=0x4b
        SpriteSheetId.x30_MOLDORM_BOSS: [
            SpriteId.x9_MOLDORM_BOSS,
        ],  # 76=0x4c
        SpriteSheetId.x31_LANMOLAS_BOSS: [
            SpriteId.x54_LANMOLAS_BOSS,
        ],  # 75=0x4b  99=0x63
        SpriteSheetId.x33_BIG_BAD_GUY: [
            SpriteId.xD6_GANON,
            SpriteId.xD7_GANON_INVINCIBLE,
        ],  # 98=0x62
        SpriteSheetId.x36_MASTER_SWORD: [
            SpriteId.x59_BIRD,
            SpriteId.x5A_SQUIRREL,
            SpriteId.x62_MASTER_SWORD,
            SpriteId.x72_FAIRY_POND_ITEM_TRIGGER,
        ],
        SpriteSheetId.x37_MASTER_SWORD: [
            SpriteId.x2B_TENTMAN,
            SpriteId.x62_MASTER_SWORD,  # Requires multiple Spritesets.
        ],
        SpriteSheetId.x38_MOTHULA_BOSS: [
            SpriteId.x88_MOTHULA_BOSS,
            SpriteId.x89_MOTHULAS_BEAM,
        ],  # 90=0x5a
        SpriteSheetId.x39_ARRGHUS_BOSS: [
            SpriteId.x8C_ARRGHUS_BOSS,
            SpriteId.x8D_ARRGHUS_SPAWN,
            SpriteId.xC8_GREAT_FAIRY,
        ],
        SpriteSheetId.x3A_HELMASAUR_KING_BOSS: [
            SpriteId.x70_HELMASAUR_KING_FIREBALL,
            SpriteId.x92_HELMASAUR_KING,
        ],
        SpriteSheetId.x3B_BLIND_BOSS: [SpriteId.xCE_BLIND_THE_THIEF_BOSS],
        SpriteSheetId.x3C_KHOLDSTARE_BOSS: [
            SpriteId.xA2_KHOLDSTARE_BOSS,
            SpriteId.xA3_KHOLDSTARES_SHELL,
        ],
        SpriteSheetId.x3D_VITREOUS_BOSS: [
            SpriteId.xBF_VITREOUS_LIGHTNING,
            SpriteId.xBD_VITREOUS_LARGE_EYEBALL,
            SpriteId.xBE_VITREOUS_SMALL_EYEBALL,
        ],
        SpriteSheetId.x3E_HELMASAUR_KING_BOSS: [
            SpriteId.x92_HELMASAUR_KING,
            SpriteId.x70_HELMASAUR_KING_FIREBALL,
        ],  # 85=0x55
        SpriteSheetId.x3F_TRINEXX_BOSS: [
            SpriteId.xCB_TRINEXX_1,
            SpriteId.xCC_TRINEXX_2,
            SpriteId.xCD_TRINEXX_3,
        ],
        SpriteSheetId.x40_TRINEXX_BOSS: [
            SpriteId.xCB_TRINEXX_1,
            SpriteId.xCC_TRINEXX_2,
            SpriteId.xCD_TRINEXX_3,
        ],
        SpriteSheetId.x41_BIG_BAD_GUY: [
            SpriteId.xD6_GANON,
            SpriteId.xD7_GANON_INVINCIBLE,
        ],  # 98=0x62
        SpriteSheetId.x42_AGAHNIM: [
            SpriteId.x7A_AGAHNIM,
            SpriteId.xC1_AGAHNIM_TELEPORTING,
            SpriteId.x7B_AGAHNIM_ENERGY_BALL,
            SpriteId.xAB_MAIDEN,
            SpriteId.x76_PRINCESS_ZELDA,
        ],  # 82=0x52  88=0x58
        SpriteSheetId.x43_AGAHNIM: [
            SpriteId.x7A_AGAHNIM,
            SpriteId.xC1_AGAHNIM_TELEPORTING,
            SpriteId.x7B_AGAHNIM_ENERGY_BALL,
            SpriteId.xAB_MAIDEN,
            SpriteId.x76_PRINCESS_ZELDA,
        ],  # 82=0x52  88=0x58
        SpriteSheetId.x44_ZORAS_DOMAIN: [
            SpriteId.x52_GIANT_ZORA,
            SpriteId.x56_WALKING_ZORA,
        ],  # 14=0xe
        SpriteSheetId.x45_BIG_BAD_GUY: [
            SpriteId.xD6_GANON,
            SpriteId.xD7_GANON_INVINCIBLE,
        ],  # 98=0x62
        SpriteSheetId.x46_SOLDIERS_1: [
            SpriteId.x6B_CANNON_SOLDIER,
            SpriteId.x47_GREEN_ARCHER,
            SpriteId.x6A_BALL_N_CHAIN_TROOPER,
        ],
        SpriteSheetId.x47_PRIEST: [
            SpriteId.x73_UNCLE_PRIEST,
        ],  # 70=0x46
        SpriteSheetId.x48_SOLDIER: [
            SpriteId.x46_BLUE_ARCHER,
            SpriteId.x3F_TUTORIAL_SOLDIER,
        ],
        SpriteSheetId.x49_SOLDIERS: [
            SpriteId.x41_BLUE_SWORD_SOLDIER,
            SpriteId.x42_GREEN_SWORD_SOLDIER,
            SpriteId.x43_RED_SPEAR_SOLDIER,
            SpriteId.xB9_FEUDING_FRIENDS_ON_DEATH_MOUNTAIN,
        ],
        SpriteSheetId.x4A_KAKARIKO: [
            SpriteId.x75_BOTTLE_SALESMAN,
            SpriteId.x3C_VILLAGE_KID,
            SpriteId.x2C_LUMBERJACKS,
            SpriteId.x2A_SWEEPING_LADY,
        ],
        SpriteSheetId.x4B_ARCHERY: [
            SpriteId.x65_ARCHERY_GUY,
        ],
        SpriteSheetId.x4C_SAHASRAHLA_WITCH: [
            SpriteId.x16_SAHASRAHLA,
            SpriteId.x2E_FLUTE_BOYS_NOTES,
            SpriteId.x36_WITCH,
        ],
        SpriteSheetId.x4D_OLD_MAN_MAIDEN: [
            SpriteId.xAB_MAIDEN,
            SpriteId.xAD_LOST_OLD_MAN,
            SpriteId.xB7_MAIDEN_IN_BLIND_DUNGEON,
        ],
        SpriteSheetId.x4E_UNCLE_PRIEST: [SpriteId.x73_UNCLE_PRIEST],  # 15=0xf
        SpriteSheetId.x4F_OLD_MAN_RUNNER: [
            SpriteId.x32_ANGRY_BROTHERS,
            SpriteId.x74_RUNNING_MAN,
            SpriteId.xAD_LOST_OLD_MAN,
        ],
        SpriteSheetId.x50_CUCCO_FOR_NPCS: [
            SpriteId.xB_CUCCO,
            SpriteId.x2F_RACE_HP_NPCS,
            SpriteId.x34_SCARED_GIRL,
            SpriteId.x3D_SCARED_LADIES_OUTSIDE_HOUSES,
        ],
        SpriteSheetId.x51_UNCLE_PRIEST_SICK_BOY: [
            SpriteId.x73_UNCLE_PRIEST,
            SpriteId.x1F_BUG_CATCHING_KID,
        ],  # 77=0x4d
        SpriteSheetId.x52_ANTIFAIRY_SPIKES: [
            SpriteId.x15_ANTIFAIRY,
            SpriteId.x77_ANTIFAIRY_ALTERNATE,
            SpriteId.x7D_BIG_SPIKE_TRAP,
            SpriteId.x82_ANTIFAIRY_CIRCLE,
            SpriteId.x8A_SPIKE_TRAP,
        ],  # 65=0x41  66=0x42  67=0x43  68=0x44  70=0x46  72=0x48  74=0x4a  76=0x4c  83=0x53  89=0x59  90=0x5a  91=0x5b  92=0x5c  93=0x5d  94=0x5e  96=0x60  97=0x61  100=0x64  101=0x65  103=0x67  107=0x6b
        SpriteSheetId.x53_UNKNOWN: [],  # 81=0x51  102=0x66  105=0x69
        SpriteSheetId.x55_AGAHNIM: [
            SpriteId.x7A_AGAHNIM,
            SpriteId.xC1_AGAHNIM_TELEPORTING,
            SpriteId.x7B_AGAHNIM_ENERGY_BALL,
            SpriteId.xAB_MAIDEN,
            SpriteId.x76_PRINCESS_ZELDA,
        ],
        SpriteSheetId.x5A_UNKNOWN: [],  # 69=0x45
        SpriteSheetId.x5C_UNKNOWN: [],  # 27=0x1b
        SpriteSheetId.x5D_MANTLE_CREDITS: [
            SpriteId.xEE_MANTLE
        ],  # Contains intro/outro sprites as well, but they are not listed.
    }
