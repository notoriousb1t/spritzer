from enum import IntEnum
from typing import Dict

from .SpriteSettings import SpriteSettings
from .SpriteType import SpriteType
from .SpriteVulnerability import SpriteVulnerability


class SpriteId(IntEnum):
    """Describes Entites (aka Sprites) in LTTP."""

    x0_CROW = 0
    x1_VULTURE = 1
    x2_FLYING_STALFOS_HEAD = 2
    x3_EMPTY = 3
    x4_PULL_SWITCH_GOOD = 4
    x5_PULL_SWITCH_UNUSED_1 = 5
    x6_PULL_SWITCH_TRAP = 6
    x7_PULL_SWITCH_UNUSED_2 = 7
    x8_OCTOROK_ONE_WAY = 8
    x9_MOLDORM_BOSS = 9
    xA_OCTOROK_FOUR_WAY = 10
    xB_CUCCO = 11
    xC_OCTOROK_STONE = 12
    xD_BUZZBLOB = 13
    xE_SNAPDRAGON = 14
    xF_OCTOBALLOON = 15
    x10_OCTOBALLOON_HATCHLINGS = 16
    x11_HINOX = 17
    x12_MOBLIN = 18
    x13_MINI_HELMASAUR = 19
    x14_GARGOYLES_DOMAIN_GATE = 20
    x15_ANTIFAIRY = 21
    x16_SAHASRAHLA = 22
    x17_BUSH_HOARDER = 23
    x18_MINI_MOLDORM = 24
    x19_POE = 25
    x1A_DWARVES = 26
    x1B_ARROW_IN_WALL = 27
    x1C_STATUE = 28
    x1D_WEATHERVANE = 29
    x1E_CRYSTAL_SWITCH = 30
    x1F_BUG_CATCHING_KID = 31
    x20_SLUGGULA = 32
    x21_PUSH_SWITCH = 33
    x22_ROPA = 34
    x23_RED_BARI = 35
    x24_BLUE_BARI = 36
    x25_TALKING_TREE = 37
    x26_HARDHAT_BEETLE = 38
    x27_DEADROCK = 39
    x28_STORYTELLERS = 40
    x29_BLIND_HIDEOUT_ATTENDANT = 41
    x2A_SWEEPING_LADY = 42
    x2B_TENTMAN = 43
    x2C_LUMBERJACKS = 44
    x2D_TELEPATHIC_STONES = 45
    x2E_FLUTE_BOYS_NOTES = 46
    x2F_RACE_HP_NPCS = 47
    x30_PERSON = 48
    x31_FORTUNE_TELLER = 49
    x32_ANGRY_BROTHERS = 50
    x33_PULL_FOR_RUPEES = 51
    x34_SCARED_GIRL = 52
    x35_INNKEEPER = 53
    x36_WITCH = 54
    x37_WATERFALL = 55
    x38_ARROW_TARGET = 56
    x39_AVERAGE_MIDDLE_AGED_MAN = 57
    x3A_HALF_MAGIC_BAT = 58
    x3B_DASH_ITEM = 59
    x3C_VILLAGE_KID = 60
    x3D_SCARED_LADIES_OUTSIDE_HOUSES = 61
    x3E_ROCK_HOARDER = 62
    x3F_TUTORIAL_SOLDIER = 63
    x40_LIGHTNING_LOCK = 64
    x41_BLUE_SWORD_SOLDIER = 65
    x42_GREEN_SWORD_SOLDIER = 66
    x43_RED_SPEAR_SOLDIER = 67
    x44_ASSAULT_SWORD_SOLDIER = 68
    x45_GREEN_SPEAR_SOLDIER = 69
    x46_BLUE_ARCHER = 70
    x47_GREEN_ARCHER = 71
    x48_RED_JAVELIN_SOLDIER = 72
    x49_RED_JAVELIN_SOLDIER_2 = 73
    x4A_RED_BOMB_SOLDIERS = 74
    x4B_GREEN_SOLDIER_RECRUITS = 75
    x4C_GELDMAN = 76
    x4D_TOPPO = 77
    x4E_POPO_1 = 78
    x4F_POPO_2 = 79
    x50_CANNON_BALLS = 80
    x51_ARMOS = 81
    x52_GIANT_ZORA = 82
    x53_ARMOS_KNIGHTS_BOSS = 83
    x54_LANMOLAS_BOSS = 84
    x55_FIREBALL_ZORA = 85
    x56_WALKING_ZORA = 86
    x57_DESERT_PALACE_BARRIERS = 87
    x58_CRAB = 88
    x59_BIRD = 89
    x5A_SQUIRREL = 90
    x5B_SPARK_LEFT_TO_RIGHT = 91
    x5C_SPARK_RIGHT_TO_LEFT = 92
    x5D_ROLLER_VERTICAL_MOVING_1 = 93
    x5E_ROLLER_VERTICAL_MOVING_2 = 94
    x5F_ROLLER = 95
    x60_ROLLER_HORIZONTAL_MOVING = 96
    x61_BEAMOS = 97
    x62_MASTER_SWORD = 98
    x63_DEVALANT_NON_SHOOTER = 99
    x64_DEVALANT_SHOOTER = 100
    x65_ARCHERY_GUY = 101
    x66_MOVING_CANNON_BALL_SHOOTERS_RIGHT = 102
    x67_MOVING_CANNON_BALL_SHOOTERS_LEFT = 103
    x68_MOVING_CANNON_BALL_SHOOTERS_DOWN = 104
    x69_MOVING_CANNON_BALL_SHOOTERS_UP = 105
    x6A_BALL_N_CHAIN_TROOPER = 106
    x6B_CANNON_SOLDIER = 107
    x6C_MIRROR_PORTAL = 108
    x6D_RAT = 109
    x6E_ROPE = 110
    x6F_KEESE = 111
    x70_HELMASAUR_KING_FIREBALL = 112
    x71_LEEVER = 113
    x72_FAIRY_POND_ITEM_TRIGGER = 114
    x73_UNCLE_PRIEST = 115
    x74_RUNNING_MAN = 116
    x75_BOTTLE_SALESMAN = 117
    x76_PRINCESS_ZELDA = 118
    x77_ANTIFAIRY_ALTERNATE = 119
    x78_VILLAGE_ELDER = 120
    x79_BEE = 121
    x7A_AGAHNIM = 122
    x7B_AGAHNIM_ENERGY_BALL = 123
    x7C_FLOATING_STALFOS_HEAD = 124
    x7D_BIG_SPIKE_TRAP = 125
    x7E_GURUGURU_BAR_CLOCKWISE = 126
    x7F_GURUGURU_BAR_COUNTER_CLOCKWISE = 127
    x80_WINDER = 128
    x81_WATER_TEKTITE = 129
    x82_ANTIFAIRY_CIRCLE = 130
    x83_GREEN_EYEGORE_MIMIC = 131
    x84_RED_EYEGORE_MIMIC = 132
    x85_YELLOW_STALFOS = 133
    x86_KODONGOS = 134
    x87_FLAMES = 135
    x88_MOTHULA_BOSS = 136
    x89_MOTHULAS_BEAM = 137
    x8A_SPIKE_TRAP = 138
    x8B_GIBDO = 139
    x8C_ARRGHUS_BOSS = 140
    x8D_ARRGHUS_SPAWN = 141
    x8E_TERRORPIN = 142
    x8F_SLIME = 143
    x90_WALLMASTER = 144
    x91_STALFOS_KNIGHT = 145
    x92_HELMASAUR_KING = 146
    x93_BUMPER = 147
    x94_SWIMMERS_EVIL = 148
    x95_EYE_LASER_RIGHT = 149
    x96_EYE_LASER_LEFT = 150
    x97_EYE_LASER_DOWN = 151
    x98_EYE_LASER_UP = 152
    x99_PENGATOR = 153
    x9A_KYAMERON_WATER_SPLASH = 154
    x9B_WIZZROBE = 155
    x9C_VERMIN_HORIZONTAL_PIROGUSU = 156
    x9D_VERMIN_VERTICAL_PIROGUSU = 157
    x9E_OSTRICH_HAUNTED_GROVE = 158
    x9F_FLUTE = 159
    xA0_BIRDS_HAUNTED_GROVE = 160
    xA1_FREEZOR = 161
    xA2_KHOLDSTARE_BOSS = 162
    xA3_KHOLDSTARES_SHELL = 163
    xA4_FALLING_ICE = 164
    xA5_BLUE_ZAZAK = 165
    xA6_RED_ZAZAK = 166
    xA7_STALFOS = 167
    xA8_ZIRRO_1 = 168
    xA9_ZIRRO_2 = 169
    xAA_PIKIT = 170
    xAB_MAIDEN = 171
    xAC_APPLE = 172
    xAD_LOST_OLD_MAN = 173
    xAE_DOWN_PIPE = 174
    xAF_UP_PIPE = 175
    xB0_RIGHT_PIPE = 176
    xB1_LEFT_PIPE = 177
    xB2_GOOD_BEE_AGAIN = 178
    xB3_HYLIAN_INSCRIPTION = 179
    xB4_THIEFS_CHEST_ITEM_TRIGGER = 180
    xB5_BOMB_SALESMAN = 181
    xB6_KIKI = 182
    xB7_MAIDEN_IN_BLIND_DUNGEON = 183
    xB8_MIMIC = 184
    xB9_FEUDING_FRIENDS_ON_DEATH_MOUNTAIN = 185
    xBA_WHIRLPOOL = 186
    xBB_SALESMAN = 187
    xBC_DRUNK_IN_THE_INN = 188
    xBD_VITREOUS_LARGE_EYEBALL = 189
    xBE_VITREOUS_SMALL_EYEBALL = 190
    xBF_VITREOUS_LIGHTNING = 191
    xC0_CATFISH = 192
    xC1_AGAHNIM_TELEPORTING = 193
    xC2_BOULDERS = 194
    xC3_GIBO = 195
    xC4_THIEF = 196
    xC5_MEDUSA = 197
    xC6_YOMO_MEDUSA = 198
    xC7_HOKKU_BOKKU = 199
    xC8_GREAT_FAIRY = 200
    xC9_TEKTITE = 201
    xCA_CHAIN_CHOMP = 202
    xCB_TRINEXX_1 = 203
    xCC_TRINEXX_2 = 204
    xCD_TRINEXX_3 = 205
    xCE_BLIND_THE_THIEF_BOSS = 206
    xCF_SWAMOLA = 207
    xD0_LYNEL = 208
    xD1_BUNNY_BEAM = 209
    xD2_FLOPPING_FISH = 210
    xD3_STAL = 211
    xD4_LANDMINE = 212
    xD5_DIGGING_GAME_PROPRIETOR = 213
    xD6_GANON = 214
    xD7_GANON_INVINCIBLE = 215
    xD8_GREEN_BOMB = 216
    xD9_GREEN_RUPEE = 217
    xDA_BLUE_RUPEE = 218
    xDB_RED_RUPEE = 219
    xDC_BOMB_REFILL_1 = 220
    xDD_BOMB_REFILL_4 = 221
    xDE_BOMB_REFILL_8 = 222
    xDF_SMALL_MAGIC_REFILL = 223
    xE0_FULL_MAGIC_REFILL = 224
    xE1_ARROW_REFILL_5 = 225
    xE2_ARROW_REFILL_10 = 226
    xE3_FAIRY = 227
    xE4_KEY = 228
    xE5_BIG_KEY = 229
    xE6_SHIELD = 230
    xE7_MUSHROOM = 231
    xE8_FAKE_MASTER_SWORD = 232
    xE9_MAGIC_MERCHANT = 233
    xEA_HEART_CONTAINER = 234
    xEB_HEART_PIECE = 235
    xEC_BUSHES = 236
    xED_CANE_OF_SOMARIA_PLATFORM = 237
    xEE_MANTLE = 238
    xEF_CANE_OF_SOMARIA_PLATFORM_UNUSED_1 = 239
    xF0_CANE_OF_SOMARIA_PLATFORM_UNUSED_2 = 240
    xF1_CANE_OF_SOMARIA_PLATFORM_UNUSED_3 = 241
    xF2_MEDALLION_TABLET = 242
    xF3_PERSONS_DOOR_OW_OVERLORD = 243
    xF4_FALLING_ROCKS_OW_OVERLORD = 244
    xF5_CANON_BALLS_OW_OVERLORD = 245
    xF6_UNKNOWN_F6_OW_OVERLORD = 246
    xF7_UNKNOWN_F7_OW_OVERLORD = 247
    xF8_UNKNOWN_F8_OW_OVERLORD = 248
    xF9_UNKNOWN_F9_OW_OVERLORD = 249
    xFA_BLOB_DROP_OW_OVERLORD = 250
    xFB_OVERWORLD_WALLMASTER_OW_OVERLORD_DROPS_IN_HOULIHAN = 251
    xFC_FLOOR_DROP_SQUARE_OW_OVERLORD = 252
    xFD_FLOOR_DROP_NORTH_PATH_OW_OVERLORD = 253
    xFE_FLOOR_DROP_EAST_PATH_OW_OVERLORD = 254
    x102_CANON_BALLS_EP_4_WALL_CANONBALLS = 258
    x103_CANON_BALLS_EP_ENTRY = 259
    x104_ROPE_DROP_TRAP = 260
    x105_STALFOS_HEAD_TRAP = 261
    x106_BOMB_DROP1_TRAP = 262
    x107_MOVING_FLOOR = 263
    x108_TRANSFORMER_BUNNY_BEAM = 264
    x109_WALLMASTER_OVERLORD = 265
    x10A_FLOOR_DROP_SQUARE = 266
    x10B_FLOOR_DROP_NORTH = 267
    x110_PIROGUSU_SPAWNER_RIGHT = 272
    x111_PIROGUSU_SPAWNER_LEFT = 273
    x112_PIROGUSU_SPAWNER_DOWN = 274
    x113_PIROGUSU_SPAWNER_UP = 275
    x114_FLYING_FLOOR_TILE_TRAP = 276
    x115_WIZZROBE_SPAWNER = 277
    x116_ZORO_SPAWNER = 278
    x117_FOUR_SKULL_TRAP_IN_POD_UNDER_POT = 279
    x118_STALFOS_APPEAR = 280
    x119_ARMOS_KNIGHTS_TRIGGER = 281
    x11A_BOMB_DROP2_TRAP = 282
    x141_SOLDIER_ALERTER_BLUE = 321
    x142_SOLDIER_ALERTER_GREEN = 322

    def __str__(self) -> str:
        return self.name

    @property
    def role(self) -> SpriteType:
        return self.meta().role

    @property
    def requires_weapon(self) -> bool:
        meta = self.meta()
        return meta.vulnerability in [
            SpriteVulnerability.BOW,
            SpriteVulnerability.FIRE,
            SpriteVulnerability.HAMMER,
        ]

    def meta(self) -> SpriteSettings:
        return _sprite_settings[self]


_sprite_settings: Dict[SpriteId, SpriteSettings] = {
    SpriteId.x0_CROW: SpriteSettings(is_flying=True, role=SpriteType.ENEMY),
    SpriteId.x1_VULTURE: SpriteSettings(is_flying=True, role=SpriteType.ENEMY),
    SpriteId.x2_FLYING_STALFOS_HEAD: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x3_EMPTY: SpriteSettings(),
    SpriteId.x4_PULL_SWITCH_GOOD: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x5_PULL_SWITCH_UNUSED_1: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x6_PULL_SWITCH_TRAP: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x7_PULL_SWITCH_UNUSED_2: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x8_OCTOROK_ONE_WAY: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x9_MOLDORM_BOSS: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xA_OCTOROK_FOUR_WAY: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xB_CUCCO: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xC_OCTOROK_STONE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xD_BUZZBLOB: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xE_SNAPDRAGON: SpriteSettings(role=SpriteType.ENEMY),
    # Note: This sometimes shows up in Misery Mire Overworld, but it produces
    # a catfish graphic. I kind of works even though it is technically an error.
    SpriteId.xF_OCTOBALLOON: SpriteSettings(role=SpriteType.ENEMY, is_flying=True),
    SpriteId.x10_OCTOBALLOON_HATCHLINGS: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x11_HINOX: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x12_MOBLIN: SpriteSettings(role=SpriteType.ENEMY, can_hold_key=False),
    SpriteId.x13_MINI_HELMASAUR: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x14_GARGOYLES_DOMAIN_GATE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x15_ANTIFAIRY: SpriteSettings(
        vulnerability=SpriteVulnerability.INVULNERABLE, role=SpriteType.ENEMY
    ),
    SpriteId.x16_SAHASRAHLA: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x17_BUSH_HOARDER: SpriteSettings(
        can_hold_key=False, role=SpriteType.ENEMY
    ),
    SpriteId.x18_MINI_MOLDORM: SpriteSettings(
        can_shuffle_in_room=False,  # Graphics errors in Eastern Palace
        role=SpriteType.ENEMY,
    ),
    SpriteId.x19_POE: SpriteSettings(is_flying=True, role=SpriteType.ENEMY),
    SpriteId.x1A_DWARVES: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x1B_ARROW_IN_WALL: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x1C_STATUE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x1D_WEATHERVANE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x1E_CRYSTAL_SWITCH: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x1F_BUG_CATCHING_KID: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x20_SLUGGULA: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x21_PUSH_SWITCH: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x22_ROPA: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x23_RED_BARI: SpriteSettings(
        is_flying=True,
        can_hold_key=False,  # They split apart and do not drop the key.
        role=SpriteType.ENEMY,
    ),
    SpriteId.x24_BLUE_BARI: SpriteSettings(role=SpriteType.ENEMY, is_flying=True),
    SpriteId.x25_TALKING_TREE: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x26_HARDHAT_BEETLE: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x27_DEADROCK: SpriteSettings(
        vulnerability=SpriteVulnerability.INVULNERABLE, role=SpriteType.ENEMY
    ),
    SpriteId.x28_STORYTELLERS: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x29_BLIND_HIDEOUT_ATTENDANT: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x2A_SWEEPING_LADY: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x2B_TENTMAN: SpriteSettings(),
    SpriteId.x2C_LUMBERJACKS: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x2D_TELEPATHIC_STONES: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x2E_FLUTE_BOYS_NOTES: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x2F_RACE_HP_NPCS: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x30_PERSON: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x31_FORTUNE_TELLER: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x32_ANGRY_BROTHERS: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x33_PULL_FOR_RUPEES: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x34_SCARED_GIRL: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x35_INNKEEPER: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x36_WITCH: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x37_WATERFALL: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x38_ARROW_TARGET: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x39_AVERAGE_MIDDLE_AGED_MAN: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x3A_HALF_MAGIC_BAT: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x3B_DASH_ITEM: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x3C_VILLAGE_KID: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x3D_SCARED_LADIES_OUTSIDE_HOUSES: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x3E_ROCK_HOARDER: SpriteSettings(
        can_hold_key=False, role=SpriteType.ENEMY
    ),
    SpriteId.x3F_TUTORIAL_SOLDIER: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x40_LIGHTNING_LOCK: SpriteSettings(
        can_shuffle=False,  # This acts as a progression gate. Shuffling this may cause unpredictable results.
        role=SpriteType.ENEMY,
    ),
    SpriteId.x41_BLUE_SWORD_SOLDIER: SpriteSettings(
        vulnerability=SpriteVulnerability.ANY, role=SpriteType.ENEMY
    ),
    SpriteId.x42_GREEN_SWORD_SOLDIER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x43_RED_SPEAR_SOLDIER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x44_ASSAULT_SWORD_SOLDIER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x45_GREEN_SPEAR_SOLDIER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x46_BLUE_ARCHER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x47_GREEN_ARCHER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x48_RED_JAVELIN_SOLDIER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x49_RED_JAVELIN_SOLDIER_2: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x4A_RED_BOMB_SOLDIERS: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x4B_GREEN_SOLDIER_RECRUITS: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x4C_GELDMAN: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.ENEMY
    ),
    SpriteId.x4D_TOPPO: SpriteSettings(
        can_shuffle_in_area=False, role=SpriteType.CREATURE
    ),
    SpriteId.x4E_POPO_1: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x4F_POPO_2: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x50_CANNON_BALLS: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x51_ARMOS: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x52_GIANT_ZORA: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x53_ARMOS_KNIGHTS_BOSS: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.x54_LANMOLAS_BOSS: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.x55_FIREBALL_ZORA: SpriteSettings(
        is_aquatic=True,
        vulnerability=SpriteVulnerability.INVULNERABLE,
        role=SpriteType.ENEMY,
    ),
    SpriteId.x56_WALKING_ZORA: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x57_DESERT_PALACE_BARRIERS: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x58_CRAB: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x59_BIRD: SpriteSettings(role=SpriteType.CREATURE),
    SpriteId.x5A_SQUIRREL: SpriteSettings(role=SpriteType.CREATURE),
    SpriteId.x5B_SPARK_LEFT_TO_RIGHT: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x5C_SPARK_RIGHT_TO_LEFT: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x5D_ROLLER_VERTICAL_MOVING_1: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x5E_ROLLER_VERTICAL_MOVING_2: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x5F_ROLLER: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x60_ROLLER_HORIZONTAL_MOVING: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x61_BEAMOS: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x62_MASTER_SWORD: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x63_DEVALANT_NON_SHOOTER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x64_DEVALANT_SHOOTER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x65_ARCHERY_GUY: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x66_MOVING_CANNON_BALL_SHOOTERS_RIGHT: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.HAZARD
    ),
    SpriteId.x67_MOVING_CANNON_BALL_SHOOTERS_LEFT: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.HAZARD
    ),
    SpriteId.x68_MOVING_CANNON_BALL_SHOOTERS_DOWN: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.HAZARD
    ),
    SpriteId.x69_MOVING_CANNON_BALL_SHOOTERS_UP: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.HAZARD
    ),
    SpriteId.x6A_BALL_N_CHAIN_TROOPER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x6B_CANNON_SOLDIER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x6C_MIRROR_PORTAL: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x6D_RAT: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x6E_ROPE: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x6F_KEESE: SpriteSettings(
        is_flying=True, can_hold_key=False, role=SpriteType.ENEMY
    ),
    SpriteId.x70_HELMASAUR_KING_FIREBALL: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x71_LEEVER: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x72_FAIRY_POND_ITEM_TRIGGER: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x73_UNCLE_PRIEST: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x74_RUNNING_MAN: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x75_BOTTLE_SALESMAN: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x76_PRINCESS_ZELDA: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x77_ANTIFAIRY_ALTERNATE: SpriteSettings(
        vulnerability=SpriteVulnerability.INVULNERABLE, role=SpriteType.ENEMY
    ),
    SpriteId.x78_VILLAGE_ELDER: SpriteSettings(role=SpriteType.NPC),
    SpriteId.x79_BEE: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.x7A_AGAHNIM: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.x7B_AGAHNIM_ENERGY_BALL: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x7C_FLOATING_STALFOS_HEAD: SpriteSettings(
        is_flying=True,
        vulnerability=SpriteVulnerability.INVULNERABLE,
        role=SpriteType.ENEMY,
    ),
    SpriteId.x7D_BIG_SPIKE_TRAP: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.HAZARD
    ),
    SpriteId.x7E_GURUGURU_BAR_CLOCKWISE: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x7F_GURUGURU_BAR_COUNTER_CLOCKWISE: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x80_WINDER: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x81_WATER_TEKTITE: SpriteSettings(is_aquatic=True, role=SpriteType.ENEMY),
    SpriteId.x82_ANTIFAIRY_CIRCLE: SpriteSettings(
        can_shuffle_in_room=False,  # This is tied to Dungeon Room mechanics (DungeonTags)
        role=SpriteType.HAZARD,
    ),
    SpriteId.x83_GREEN_EYEGORE_MIMIC: SpriteSettings(
        vulnerability=SpriteVulnerability.ANY, role=SpriteType.ENEMY
    ),
    SpriteId.x84_RED_EYEGORE_MIMIC: SpriteSettings(
        vulnerability=SpriteVulnerability.BOW, role=SpriteType.ENEMY
    ),
    SpriteId.x85_YELLOW_STALFOS: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x86_KODONGOS: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x87_FLAMES: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x88_MOTHULA_BOSS: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.x89_MOTHULAS_BEAM: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.x8A_SPIKE_TRAP: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.x8B_GIBDO: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x8C_ARRGHUS_BOSS: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.x8D_ARRGHUS_SPAWN: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.ENEMY
    ),
    SpriteId.x8E_TERRORPIN: SpriteSettings(
        vulnerability=SpriteVulnerability.HAMMER, role=SpriteType.ENEMY
    ),
    SpriteId.x8F_SLIME: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x90_WALLMASTER: SpriteSettings(
        can_shuffle=False, role=SpriteType.OVERLORD
    ),
    SpriteId.x91_STALFOS_KNIGHT: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x92_HELMASAUR_KING: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.x93_BUMPER: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.HAZARD
    ),
    SpriteId.x94_SWIMMERS_EVIL: SpriteSettings(is_aquatic=True, role=SpriteType.ENEMY),
    SpriteId.x95_EYE_LASER_RIGHT: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.HAZARD  # Needs directionality
    ),
    SpriteId.x96_EYE_LASER_LEFT: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.HAZARD  # Needs directionality
    ),
    SpriteId.x97_EYE_LASER_DOWN: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.HAZARD  # Needs directionality
    ),
    SpriteId.x98_EYE_LASER_UP: SpriteSettings(
        role=SpriteType.HAZARD,
        can_shuffle_in_room=False,  # Needs directionality
    ),
    SpriteId.x99_PENGATOR: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x9A_KYAMERON_WATER_SPLASH: SpriteSettings(
        role=SpriteType.ENEMY, vulnerability=SpriteVulnerability.INVULNERABLE
    ),
    SpriteId.x9B_WIZZROBE: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.x9C_VERMIN_HORIZONTAL_PIROGUSU: SpriteSettings(
        can_hold_key=False, can_shuffle_in_room=False, role=SpriteType.ENEMY
    ),
    SpriteId.x9D_VERMIN_VERTICAL_PIROGUSU: SpriteSettings(
        can_hold_key=False, can_shuffle_in_room=False, role=SpriteType.ENEMY
    ),
    SpriteId.x9E_OSTRICH_HAUNTED_GROVE: SpriteSettings(role=SpriteType.CREATURE),
    SpriteId.x9F_FLUTE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xA0_BIRDS_HAUNTED_GROVE: SpriteSettings(role=SpriteType.CREATURE),
    SpriteId.xA1_FREEZOR: SpriteSettings(
        vulnerability=SpriteVulnerability.FIRE,
        can_shuffle_in_room=False,  # Collision box can block progress in Misery Mire.
        role=SpriteType.ENEMY,
    ),
    SpriteId.xA2_KHOLDSTARE_BOSS: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xA3_KHOLDSTARES_SHELL: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xA4_FALLING_ICE: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.xA5_BLUE_ZAZAK: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xA6_RED_ZAZAK: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xA7_STALFOS: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xA8_ZIRRO_1: SpriteSettings(can_hold_key=False, role=SpriteType.ENEMY),
    SpriteId.xA9_ZIRRO_2: SpriteSettings(
        can_hold_key=False, is_flying=True, role=SpriteType.ENEMY
    ),
    SpriteId.xAA_PIKIT: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xAB_MAIDEN: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xAC_APPLE: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xAD_LOST_OLD_MAN: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xAE_DOWN_PIPE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xAF_UP_PIPE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xB0_RIGHT_PIPE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xB1_LEFT_PIPE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xB2_GOOD_BEE_AGAIN: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xB3_HYLIAN_INSCRIPTION: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xB4_THIEFS_CHEST_ITEM_TRIGGER: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xB5_BOMB_SALESMAN: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xB6_KIKI: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xB7_MAIDEN_IN_BLIND_DUNGEON: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xB8_MIMIC: SpriteSettings(
        vulnerability=SpriteVulnerability.BOW, role=SpriteType.ENEMY
    ),
    SpriteId.xB9_FEUDING_FRIENDS_ON_DEATH_MOUNTAIN: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xBA_WHIRLPOOL: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xBB_SALESMAN: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xBC_DRUNK_IN_THE_INN: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xBD_VITREOUS_LARGE_EYEBALL: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xBE_VITREOUS_SMALL_EYEBALL: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xBF_VITREOUS_LIGHTNING: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xC0_CATFISH: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xC1_AGAHNIM_TELEPORTING: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xC2_BOULDERS: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xC3_GIBO: SpriteSettings(role=SpriteType.ENEMY),
    # Note: This shows up as a "smurf" due to a wrong palette in lumberjack Overworld Area.
    SpriteId.xC4_THIEF: SpriteSettings(
        vulnerability=SpriteVulnerability.INVULNERABLE, role=SpriteType.ENEMY
    ),
    SpriteId.xC5_MEDUSA: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.xC6_YOMO_MEDUSA: SpriteSettings(role=SpriteType.HAZARD),
    SpriteId.xC7_HOKKU_BOKKU: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xC8_GREAT_FAIRY: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xC9_TEKTITE: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xCA_CHAIN_CHOMP: SpriteSettings(
        vulnerability=SpriteVulnerability.INVULNERABLE, role=SpriteType.ENEMY
    ),
    SpriteId.xCB_TRINEXX_1: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xCC_TRINEXX_2: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xCD_TRINEXX_3: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xCE_BLIND_THE_THIEF_BOSS: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xCF_SWAMOLA: SpriteSettings(
        is_aquatic=True, can_shuffle=False, role=SpriteType.ENEMY
    ),
    SpriteId.xD0_LYNEL: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xD1_BUNNY_BEAM: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xD2_FLOPPING_FISH: SpriteSettings(role=SpriteType.CREATURE),
    SpriteId.xD3_STAL: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xD4_LANDMINE: SpriteSettings(role=SpriteType.ENEMY),
    SpriteId.xD5_DIGGING_GAME_PROPRIETOR: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xD6_GANON: SpriteSettings(role=SpriteType.BOSS),
    SpriteId.xD7_GANON_INVINCIBLE: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xD8_GREEN_BOMB: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xD9_GREEN_RUPEE: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xDA_BLUE_RUPEE: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xDB_RED_RUPEE: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xDC_BOMB_REFILL_1: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xDD_BOMB_REFILL_4: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xDE_BOMB_REFILL_8: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xDF_SMALL_MAGIC_REFILL: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xE0_FULL_MAGIC_REFILL: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xE1_ARROW_REFILL_5: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xE2_ARROW_REFILL_10: SpriteSettings(role=SpriteType.CONSUMABLE),
    SpriteId.xE3_FAIRY: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.CONSUMABLE
    ),
    SpriteId.xE4_KEY: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xE5_BIG_KEY: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xE6_SHIELD: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xE7_MUSHROOM: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xE8_FAKE_MASTER_SWORD: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xE9_MAGIC_MERCHANT: SpriteSettings(role=SpriteType.NPC),
    SpriteId.xEA_HEART_CONTAINER: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xEB_HEART_PIECE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xEC_BUSHES: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xED_CANE_OF_SOMARIA_PLATFORM: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xEE_MANTLE: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xEF_CANE_OF_SOMARIA_PLATFORM_UNUSED_1: SpriteSettings(
        role=SpriteType.OBJECT
    ),
    SpriteId.xF0_CANE_OF_SOMARIA_PLATFORM_UNUSED_2: SpriteSettings(
        role=SpriteType.OBJECT
    ),
    SpriteId.xF1_CANE_OF_SOMARIA_PLATFORM_UNUSED_3: SpriteSettings(
        role=SpriteType.OBJECT
    ),
    SpriteId.xF2_MEDALLION_TABLET: SpriteSettings(role=SpriteType.OBJECT),
    SpriteId.xF3_PERSONS_DOOR_OW_OVERLORD: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.xF4_FALLING_ROCKS_OW_OVERLORD: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.xF5_CANON_BALLS_OW_OVERLORD: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.xF6_UNKNOWN_F6_OW_OVERLORD: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.OVERLORD
    ),
    SpriteId.xF7_UNKNOWN_F7_OW_OVERLORD: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.OVERLORD
    ),
    SpriteId.xF8_UNKNOWN_F8_OW_OVERLORD: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.OVERLORD
    ),
    SpriteId.xF9_UNKNOWN_F9_OW_OVERLORD: SpriteSettings(
        can_shuffle_in_room=False, role=SpriteType.OVERLORD
    ),
    SpriteId.xFA_BLOB_DROP_OW_OVERLORD: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.xFB_OVERWORLD_WALLMASTER_OW_OVERLORD_DROPS_IN_HOULIHAN: SpriteSettings(
        role=SpriteType.OVERLORD
    ),
    SpriteId.xFC_FLOOR_DROP_SQUARE_OW_OVERLORD: SpriteSettings(
        role=SpriteType.OVERLORD
    ),
    SpriteId.xFD_FLOOR_DROP_NORTH_PATH_OW_OVERLORD: SpriteSettings(
        role=SpriteType.OVERLORD
    ),
    SpriteId.xFE_FLOOR_DROP_EAST_PATH_OW_OVERLORD: SpriteSettings(
        role=SpriteType.OVERLORD
    ),
    SpriteId.x102_CANON_BALLS_EP_4_WALL_CANONBALLS: SpriteSettings(
        role=SpriteType.OVERLORD
    ),
    SpriteId.x103_CANON_BALLS_EP_ENTRY: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x106_BOMB_DROP1_TRAP: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x104_ROPE_DROP_TRAP: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x105_STALFOS_HEAD_TRAP: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x11A_BOMB_DROP2_TRAP: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x107_MOVING_FLOOR: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x108_TRANSFORMER_BUNNY_BEAM: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x109_WALLMASTER_OVERLORD: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x10A_FLOOR_DROP_SQUARE: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x10B_FLOOR_DROP_NORTH: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x110_PIROGUSU_SPAWNER_RIGHT: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x111_PIROGUSU_SPAWNER_LEFT: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x112_PIROGUSU_SPAWNER_DOWN: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x113_PIROGUSU_SPAWNER_UP: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x114_FLYING_FLOOR_TILE_TRAP: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x115_WIZZROBE_SPAWNER: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x116_ZORO_SPAWNER: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x117_FOUR_SKULL_TRAP_IN_POD_UNDER_POT: SpriteSettings(
        role=SpriteType.OVERLORD
    ),
    SpriteId.x118_STALFOS_APPEAR: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x119_ARMOS_KNIGHTS_TRIGGER: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x11A_BOMB_DROP2_TRAP: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x141_SOLDIER_ALERTER_BLUE: SpriteSettings(role=SpriteType.OVERLORD),
    SpriteId.x142_SOLDIER_ALERTER_GREEN: SpriteSettings(role=SpriteType.OVERLORD),
}


