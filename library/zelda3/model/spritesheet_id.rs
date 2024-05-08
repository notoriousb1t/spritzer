use strum_macros::Display;
use strum_macros::FromRepr;

#[repr(u8)]
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, FromRepr, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub(crate) enum SpritesheetId {
    None = 0,
    X1 = 1,
    X2 = 2,
    X3 = 3,
    X4 = 4,
    X5 = 5,
    X6 = 6,
    X7 = 7,
    X8 = 8,
    X9 = 9,
    XA = 10,
    XB = 11,
    xC_OCTOROK_ZORA = 12,
    xD_SOLDIERS_DW = 13,
    xE_POE_THIEF_LW = 14,
    xF_DASH_HOARDER = 15,
    x10_MISC_ROCKS = 16,
    x11_MISC_FAKE_SWORD = 17,
    x12_DESERT_1 = 18,
    x13_SOLDIER_RECRUITS = 19,
    x14_FRIENDLY_LYNEL = 20,
    x15_POE_THIEF_DW = 21,
    x16_HINOX_SNAPDRAGON = 22,
    x17_MOBLIN = 23,
    x18_OCTOROCKS = 24,
    x19_SWAMOLA_CROW = 25,
    x1A_AGAHNIM = 26,
    x1B_MISCELLANEOUS_DW_1 = 27,
    x1C_PESTS = 28,
    x1D_ARMOS_BOSS_LOCK_BAT = 29,
    x1E_MINI_MONSTERS = 30,
    x1F_STALFOS_BARI = 31,
    x20_STALFOS_KNIGHT_VERMIN = 32,
    x21_BIG_BAD_GUY = 33,
    x22_WATER_TEKTITES = 34,
    x23_WALLMASTER_GIBDO = 35,
    x24_PESTS_DW = 36,
    x25_WIZZROBE_SLUGGULA = 37,
    x26_FROSTY_FRIENDS = 38,
    x27_TURTLE_ROCK = 39,
    x28_ZAZAK = 40,
    x29_WIZZROBE = 41,
    x2A_HAZARDS = 42,
    x2B_UNUSED = 43,
    x2C_BEAM_ME_UP_MR_POPO = 44,
    x2D_UNKNOWN = 45,
    x2E_EyeGoreCannonBall = 46,
    x2F_CANON_SANDCRAB = 47,
    x30_MOLDORM_BOSS = 48,
    x31_LANMOLAS_BOSS = 49,
    x32 = 50,
    x33_BIG_BAD_GUY = 51,
    x34 = 52,
    x35 = 53,
    x36_MASTER_SWORD = 54,
    x37_MASTER_SWORD = 55,
    x38_MOTHULA_BOSS = 56,
    x39_ARRGHUS_BOSS = 57,
    x3A_HELMASAUR_KING_BOSS = 58,
    x3B_BLIND_BOSS = 59,
    x3C_KHOLDSTARE_BOSS = 60,
    x3D_VITREOUS_BOSS = 61,
    x3E_HELMASAUR_KING_BOSS = 62,
    x3F_TRINEXX_BOSS = 63,
    x40_TRINEXX_BOSS = 64,
    x41_BIG_BAD_GUY = 65,
    x42_AGAHNIM = 66,
    x43_AGAHNIM = 67,
    x44_ZORAS_DOMAIN = 68,
    x45_BIG_BAD_GUY = 69,
    X46_ELITE_GUARD = 70,
    x47_PRIEST = 71,
    x48_SOLDIER = 72,
    x49_SOLDIERS = 73,
    x4A_KAKARIKO = 74,
    x4B_ARCHERY = 75,
    x4C_SAHASRAHLA_WITCH = 76,
    x4D_OLD_MAN_MAIDEN = 77,
    x4E_OSTR_DUCK_RABBIT_FLUTEBOY = 78,
    x4F_OLD_MAN_RUNNER = 79,
    x50_CUCCO_FOR_NPCS = 80,
    x51_UNCLE_PRIEST_SICK_BOY = 81,
    x52_ObjectsHazards = 82,
    x53_ObjectsHazards = 83,
    x54_PushBlockSpike = 84,
    x55_AgahnimCrystalMaiden = 85,
    x56_Map = 86,
    x57_Map = 87,
    x58_FluteBirdChestBomb = 88,
    x59_FollowerKiki = 89,
    x5A_Items = 90,
    x5B_Items = 91,
    x5C_Items = 92,
    x5D_ObjectsFollowerThief = 93,
    x5E_Swords = 94,
    x5F_Items = 95,
    x60_Items = 96,
    x61_Map = 97,
    x62_Map = 98,
    x63_Map = 99,
    x64_FollowerZelda = 100,
    x65_FollowerOldManMaiden = 101,
}
