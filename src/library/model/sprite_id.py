from enum import IntEnum
from typing import Dict

from library.model.sprite_movement import SpriteMovement
from library.model.sprite_configuration import SpriteConfiguration
from library.model.sprite_type import SpriteType
from library.model.sprite_vulnerability import SpriteVulnerability


class SpriteId(IntEnum):
    """Describes Sprites (aka Entities) in ALTTP."""

    x0_RAVEN = 0
    x1_VULTURE = 1
    x2_STALFOS_HEAD = 2
    x3_NONE = 3
    x4_PULL_SWITCH_NORMAL = 4
    x5_PULL_SWITCH_NORMAL_UNUSED = 5
    x6_PULL_SWITCH_TRAP = 6
    x7_PULL_SWITCH_TRAP_UNUSED = 7
    x8_OCTOROK = 8
    x9_MOLDORM = 9
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
    x14_THIEVES_TOWN_GRATE = 20
    x15_ANTIFAIRY = 21
    x16_SAHASRAHLA = 22
    x17_HOARDER = 23
    x18_MINI_MOLDORM = 24
    x19_POE = 25
    x1A_SMITHY = 26
    x1B_ARROW = 27
    x1C_STATUE = 28
    x1D_FLUTEQUEST = 29
    x1E_CRYSTAL_SWITCH = 30
    x1F_BUG_CATCHING_KID = 31
    x20_SLUGGULA = 32
    x21_WATER_SWITCH = 33
    x22_ROPA = 34
    x23_RED_BARI = 35
    x24_BLUE_BARI = 36
    x25_TALKING_TREE = 37
    x26_HARDHAT_BEETLE = 38
    x27_DEADROCK = 39
    x28_HINT_PC_DW = 40
    x29_BLIND_HIDEOUT_ATTENDANT = 41
    x2A_SWEEPING_LADY = 42
    x2B_TENTMAN = 43
    x2C_LUMBERJACKS = 44
    x2D_NECKLESS_MAN = 45
    x2E_FLUTE_KID = 46
    x2F_RACE_LADY = 47
    x30_RACE_GUY = 48
    x31_FORTUNE_TELLER = 49
    x32_ANGRY_BROTHERS = 50
    x33_RUPEE_PULL = 51
    x34_SNITCH_YOUNG = 52
    x35_INNKEEPER = 53
    x36_WITCH = 54
    x37_WATERFALL = 55
    x38_EYEGORE_STATUE = 56
    x39_LOCKSMITH = 57
    x3A_MAGIC_BAT = 58
    x3B_BONK_ITEM = 59
    x3C_VILLAGE_KID = 60
    x3D_SNITCH_OLD = 61
    x3E_HOARDER_ROCK = 62
    x3F_TUTORIAL_SOLDIER = 63
    x40_LIGHTNING_LOCK = 64
    x41_BLUE_GUARD = 65
    x42_GREEN_GUARD = 66
    x43_RED_SPEAR_GUARD = 67
    x44_BLUE_ASSAULT_GUARD = 68
    x45_GREEN_ASSAULT_GUARD = 69
    x46_BLUE_ARCHER = 70
    x47_GREEN_GUARD_BUSH = 71
    x48_RED_JAVELIN_GUARD = 72
    x49_RED_GUARD_BUSH = 73
    x4A_RED_BOMB_GUARD = 74
    x4B_GREEN_KNIFE_GUARD = 75
    x4C_GELDMAN = 76
    x4D_TOPPO = 77
    x4E_POPO_1 = 78
    x4F_POPO_2 = 79
    x50_CANNON_BALL = 80
    x51_ARMOS_STATUE = 81
    x52_ZORA_KING = 82
    x53_ARMOS_KNIGHT = 83
    x54_LANMOLAS = 84
    x55_FIREBALL_ZORA = 85
    x56_WALKING_ZORA = 86
    x57_DESERT_STATUE = 87
    x58_CRAB = 88
    x59_LOST_WOODS_BIRD = 89
    x5A_LOST_WOODS_SQUIRREL = 90
    x5B_SPARK_LEFT_TO_RIGHT = 91
    x5C_SPARK_RIGHT_TO_LEFT = 92
    x5D_ROLLER_SOUTH = 93
    x5E_ROLLER_NORTH = 94
    x5F_ROLLER_EAST = 95
    x60_ROLLER_WEST = 96
    x61_BEAMOS = 97
    x62_MASTERSWORD = 98
    x63_DEVALANT_PIT = 99
    x64_DEVALANT = 100
    x65_ARCHERY_GUY = 101
    x66_MOVING_CANNON_WEST = 102
    x67_MOVING_CANNON_EAST = 103
    x68_MOVING_CANNON_SOUTH = 104
    x69_MOVING_CANNON_NORTH = 105
    x6A_BALL_N_CHAIN_GUARD = 106
    x6B_CANNON_GUARD = 107
    x6C_MIRROR_PORTAL = 108
    x6D_RAT_CRICKET = 109
    x6E_ROPE = 110
    x6F_KEESE = 111
    x70_HELMASAUR_KING_FIREBALL = 112
    x71_LEEVER = 113
    x72_FAIRY_POND_TRIGGER = 114
    x73_UNCLE_PRIEST_MANTLE = 115
    x74_RUNNING_MAN = 116
    x75_BOTTLE_SALESMAN = 117
    x76_ZELDA = 118
    x77_ANTIFAIRY_2 = 119
    x78_VILLAGE_ELDER = 120
    x79_BEE = 121
    x7A_AGAHNIM = 122
    x7B_AGAHNIM_ENERGY_BALL = 123
    x7C_GREEN_STALFOS = 124
    x7D_BIG_SPIKE = 125
    x7E_FIREBAR_CLOCKWISE = 126
    x7F_FIREBAR_COUNTER_CLOCKWISE = 127
    x80_FIRESNAKE = 128
    x81_WATER_TEKTITE = 129
    x82_ANTIFAIRY_CIRCLE = 130
    x83_GREEN_EYEGORE_MIMIC = 131
    x84_RED_EYEGORE_MIMIC = 132
    x85_YELLOW_STALFOS = 133
    x86_KODONGO = 134
    x87_KODONGO_FIRE = 135
    x88_MOTHULA = 136
    x89_MOTHULA_BEAM = 137
    x8A_SPIKE_BLOCK = 138
    x8B_GIBDO = 139
    x8C_ARRGHUS = 140
    x8D_ARRGHUS_SPAWN = 141
    x8E_TERRORPIN = 142
    x8F_BLOB = 143
    x90_WALLMASTER = 144
    x91_STALFOS_KNIGHT = 145
    x92_KING_HELMASAUR = 146
    x93_BUMPER = 147
    x94_PIROGUSU = 148
    x95_EYE_LASER_EAST = 149
    x96_EYE_LASER_WEST = 150
    x97_EYE_LASER_SOUTH = 151
    x98_EYE_LASER_NORTH = 152
    x99_PENGATOR = 153
    x9A_KYAMERON = 154
    x9B_WIZZROBE = 155
    x9C_BABASU_SOUTH = 156
    x9D_BABUSU_EAST = 157
    x9E_HAUNTED_GROVE_OSTRICH = 158
    x9F_HAUNTED_GROVE_RABBIT = 159
    xA0_HAUNTED_GROVE_BIRD = 160
    xA1_FREEZOR = 161
    xA2_KHOLDSTARE = 162
    xA3_KHOLDSTARES_SHELL = 163
    xA4_FALLING_ICE = 164
    xA5_BLUE_ZAZAK = 165
    xA6_RED_ZAZAK = 166
    xA7_STALFOS = 167
    xA8_GREEN_ZIRRO = 168
    xA9_BLUE_ZIRRO = 169
    xAA_PIKIT_LIKE_LIKE = 170
    xAB_CRYSTAL_MAIDEN = 171
    xAC_APPLE = 172
    xAD_LOST_OLD_MAN = 173
    xAE_DOWN_PIPE = 174
    xAF_UP_PIPE = 175
    xB0_RIGHT_PIPE = 176
    xB1_LEFT_PIPE = 177
    xB2_GOOD_BEE = 178
    xB3_PEDESTAL_PLAQUE = 179
    xB4_PURPLE_CHEST = 180
    xB5_BOMB_SALESMAN = 181
    xB6_KIKI = 182
    xB7_BLIND_MAIDEN = 183
    xB8_DIALOGUE_TESTER = 184
    xB9_BULLY_AND_FRIEND = 185
    xBA_WHIRLPOOL = 186
    xBB_SALESMAN = 187
    xBC_DRUNK_IN_THE_INN = 188
    xBD_VITREOUS = 189
    xBE_VITREOUS_SMALL_EYEBALL = 190
    xBF_LIGHTNING = 191
    xC0_CATFISH = 192
    xC1_AGAHNIM_TELEPORTING = 193
    xC2_BOULDER = 194
    xC3_GIBO = 195
    xC4_THIEF = 196
    xC5_MEDUSA = 197
    xC6_MEDUSA_FOUR_WAY = 198
    xC7_POKEY = 199
    xC8_GREAT_FAIRY = 200
    xC9_TEKTITE = 201
    xCA_CHAIN_CHOMP = 202
    xCB_TRINEXX_ROCK = 203
    xCC_TRINEXX_FIRE = 204
    xCD_TRINEXX_ICE = 205
    xCE_BLIND = 206
    xCF_SWAMOLA = 207
    xD0_LYNEL = 208
    xD1_BUNNY_BEAM = 209
    xD2_FLOPPING_FISH = 210
    xD3_STAL = 211
    xD4_LANDMINE = 212
    xD5_DIGGING_GAME_PROPRIETOR = 213
    xD6_GANON = 214
    xD7_GANON_INVINCIBLE = 215
    xD8_HEART = 216
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
    xE4_SMALL_KEY = 228
    xE5_BIG_KEY = 229
    xE6_SHIELD = 230
    xE7_MUSHROOM = 231
    xE8_FAKE_MASTER_SWORD = 232
    xE9_MAGIC_MERCHANT = 233
    xEA_HEART_CONTAINER = 234
    xEB_HEART_PIECE = 235
    xEC_THROWN_ITEM = 236
    xED_SOMARIA_PLATFORM = 237
    xEE_CASTLE_MANTLE = 238
    xEF_SOMARIA_PLATFORM_UNUSED_1 = 239
    xF0_SOMARIA_PLATFORM_UNUSED_2 = 240
    xF1_SOMARIA_PLATFORM_UNUSED_3 = 241
    xF2_MEDALLION_TABLET = 242

    # Start Overlords.
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
        return self.configuration().role

    @property
    def requires_weapon(self) -> bool:
        meta: SpriteConfiguration = self.configuration()
        return meta.vulnerability in [
            SpriteVulnerability.BOW,
            SpriteVulnerability.FIRE,
            SpriteVulnerability.HAMMER,
        ]

    def configuration(self) -> SpriteConfiguration:
        return _sprite_configuration[self]


_sprite_configuration: Dict[SpriteId, SpriteConfiguration] = {
    SpriteId.x0_RAVEN: SpriteConfiguration(is_flying=True, role=SpriteType.ENEMY),
    SpriteId.x1_VULTURE: SpriteConfiguration(is_flying=True, role=SpriteType.ENEMY),
    SpriteId.x2_STALFOS_HEAD: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x3_NONE: SpriteConfiguration(),
    SpriteId.x4_PULL_SWITCH_NORMAL: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x5_PULL_SWITCH_NORMAL_UNUSED: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x6_PULL_SWITCH_TRAP: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x7_PULL_SWITCH_TRAP_UNUSED: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x8_OCTOROK: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x9_MOLDORM: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xA_OCTOROK_FOUR_WAY: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xB_CUCCO: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xC_OCTOROK_STONE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xD_BUZZBLOB: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xE_SNAPDRAGON: SpriteConfiguration(role=SpriteType.ENEMY),
    # Note: This sometimes shows up in Misery Mire Overworld, but it produces
    # a catfish graphic. I kind of works even though it is technically an error.
    SpriteId.xF_OCTOBALLOON: SpriteConfiguration(role=SpriteType.ENEMY, is_flying=True),
    SpriteId.x10_OCTOBALLOON_HATCHLINGS: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x11_HINOX: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x12_MOBLIN: SpriteConfiguration(role=SpriteType.ENEMY, can_hold_key=False),
    SpriteId.x13_MINI_HELMASAUR: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x14_THIEVES_TOWN_GRATE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x15_ANTIFAIRY: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.DIAGONAL,
    ),
    SpriteId.x16_SAHASRAHLA: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x17_HOARDER: SpriteConfiguration(
        can_hold_key=False, role=SpriteType.ENEMY
    ),
    SpriteId.x18_MINI_MOLDORM: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x19_POE: SpriteConfiguration(is_flying=True, role=SpriteType.ENEMY),
    SpriteId.x1A_SMITHY: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x1B_ARROW: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x1C_STATUE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x1D_FLUTEQUEST: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x1E_CRYSTAL_SWITCH: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x1F_BUG_CATCHING_KID: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x20_SLUGGULA: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x21_WATER_SWITCH: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x22_ROPA: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x23_RED_BARI: SpriteConfiguration(
        is_flying=True,
        can_hold_key=False,  # They split apart and do not drop the key.
        role=SpriteType.ENEMY,
    ),
    SpriteId.x24_BLUE_BARI: SpriteConfiguration(role=SpriteType.ENEMY, is_flying=True),
    SpriteId.x25_TALKING_TREE: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x26_HARDHAT_BEETLE: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x27_DEADROCK: SpriteConfiguration(
        vulnerability=SpriteVulnerability.INVULNERABLE, role=SpriteType.ENEMY
    ),
    SpriteId.x28_HINT_PC_DW: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x29_BLIND_HIDEOUT_ATTENDANT: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x2A_SWEEPING_LADY: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x2B_TENTMAN: SpriteConfiguration(),
    SpriteId.x2C_LUMBERJACKS: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x2D_NECKLESS_MAN: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x2E_FLUTE_KID: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x2F_RACE_LADY: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x30_RACE_GUY: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x31_FORTUNE_TELLER: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x32_ANGRY_BROTHERS: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x33_RUPEE_PULL: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x34_SNITCH_YOUNG: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x35_INNKEEPER: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x36_WITCH: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x37_WATERFALL: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x38_EYEGORE_STATUE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x39_LOCKSMITH: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x3A_MAGIC_BAT: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x3B_BONK_ITEM: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x3C_VILLAGE_KID: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x3D_SNITCH_OLD: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x3E_HOARDER_ROCK: SpriteConfiguration(
        can_hold_key=False, role=SpriteType.ENEMY
    ),
    SpriteId.x3F_TUTORIAL_SOLDIER: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x40_LIGHTNING_LOCK: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x41_BLUE_GUARD: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x42_GREEN_GUARD: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x43_RED_SPEAR_GUARD: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x44_BLUE_ASSAULT_GUARD: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x45_GREEN_ASSAULT_GUARD: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x46_BLUE_ARCHER: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x47_GREEN_GUARD_BUSH: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x48_RED_JAVELIN_GUARD: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x49_RED_GUARD_BUSH: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x4A_RED_BOMB_GUARD: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x4B_GREEN_KNIFE_GUARD: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x4C_GELDMAN: SpriteConfiguration(
        role=SpriteType.ENEMY, can_hold_key=False
    ),
    SpriteId.x4D_TOPPO: SpriteConfiguration(
        can_shuffle_in_area=False, role=SpriteType.CREATURE
    ),
    SpriteId.x4E_POPO_1: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x4F_POPO_2: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x50_CANNON_BALL: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x51_ARMOS_STATUE: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x52_ZORA_KING: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x53_ARMOS_KNIGHT: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.x54_LANMOLAS: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.x55_FIREBALL_ZORA: SpriteConfiguration(
        is_aquatic=True,
        role=SpriteType.ENEMY,
    ),
    SpriteId.x56_WALKING_ZORA: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x57_DESERT_STATUE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x58_CRAB: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x59_LOST_WOODS_BIRD: SpriteConfiguration(role=SpriteType.CREATURE),
    SpriteId.x5A_LOST_WOODS_SQUIRREL: SpriteConfiguration(role=SpriteType.CREATURE),
    SpriteId.x5B_SPARK_LEFT_TO_RIGHT: SpriteConfiguration(
        role=SpriteType.HAZARD, movement=SpriteMovement.SNAKE | SpriteMovement.FIXED
    ),
    SpriteId.x5C_SPARK_RIGHT_TO_LEFT: SpriteConfiguration(
        role=SpriteType.HAZARD, movement=SpriteMovement.SNAKE | SpriteMovement.FIXED
    ),
    SpriteId.x5D_ROLLER_SOUTH: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.SOUTH,
    ),
    SpriteId.x5E_ROLLER_NORTH: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.NORTH,
    ),
    SpriteId.x5F_ROLLER_EAST: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.EAST,
    ),
    SpriteId.x60_ROLLER_WEST: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.WEST,
    ),
    SpriteId.x61_BEAMOS: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.FIXED,
    ),
    SpriteId.x62_MASTERSWORD: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x63_DEVALANT_PIT: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.FIXED,
        can_shuffle_in_room=False,
    ),
    SpriteId.x64_DEVALANT: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x65_ARCHERY_GUY: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x66_MOVING_CANNON_WEST: SpriteConfiguration(
        can_shuffle_in_room=False,
        role=SpriteType.HAZARD,
        movement=SpriteMovement.EAST,
    ),
    SpriteId.x67_MOVING_CANNON_EAST: SpriteConfiguration(
        can_shuffle_in_room=False,
        role=SpriteType.HAZARD,
        movement=SpriteMovement.WEST,
    ),
    SpriteId.x68_MOVING_CANNON_SOUTH: SpriteConfiguration(
        can_shuffle_in_room=False,
        role=SpriteType.HAZARD,
        movement=SpriteMovement.WEST,
    ),
    SpriteId.x69_MOVING_CANNON_NORTH: SpriteConfiguration(
        can_shuffle_in_room=False,
        role=SpriteType.HAZARD,
        movement=SpriteMovement.NORTH,
    ),
    SpriteId.x6A_BALL_N_CHAIN_GUARD: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x6B_CANNON_GUARD: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x6C_MIRROR_PORTAL: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x6D_RAT_CRICKET: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x6E_ROPE: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x6F_KEESE: SpriteConfiguration(
        is_flying=True, can_hold_key=False, role=SpriteType.ENEMY
    ),
    SpriteId.x70_HELMASAUR_KING_FIREBALL: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x71_LEEVER: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x72_FAIRY_POND_TRIGGER: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x73_UNCLE_PRIEST_MANTLE: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x74_RUNNING_MAN: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x75_BOTTLE_SALESMAN: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x76_ZELDA: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x77_ANTIFAIRY_2: SpriteConfiguration(
        vulnerability=SpriteVulnerability.INVULNERABLE,
        role=SpriteType.ENEMY,
        can_shuffle_in_room=False,
        movement=SpriteMovement.DIAGONAL,
    ),
    SpriteId.x78_VILLAGE_ELDER: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.x79_BEE: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.x7A_AGAHNIM: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.x7B_AGAHNIM_ENERGY_BALL: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x7C_GREEN_STALFOS: SpriteConfiguration(
        is_flying=True,
        vulnerability=SpriteVulnerability.INVULNERABLE,
        role=SpriteType.ENEMY,
    ),
    SpriteId.x7D_BIG_SPIKE: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.HORIZONTAL | SpriteMovement.VERTICAL,
    ),
    SpriteId.x7E_FIREBAR_CLOCKWISE: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.FIXED,
    ),
    SpriteId.x7F_FIREBAR_COUNTER_CLOCKWISE: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.FIXED,
    ),
    SpriteId.x80_FIRESNAKE: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.SNAKE | SpriteMovement.DIAGONAL,
    ),
    SpriteId.x81_WATER_TEKTITE: SpriteConfiguration(
        is_aquatic=True, role=SpriteType.ENEMY
    ),
    SpriteId.x82_ANTIFAIRY_CIRCLE: SpriteConfiguration(
        can_shuffle_in_room=False,  # This is tied to Underworld Room mechanics (UnderworldRoomTags)
        role=SpriteType.HAZARD,
        movement=SpriteMovement.DIAGONAL,
    ),
    SpriteId.x83_GREEN_EYEGORE_MIMIC: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x84_RED_EYEGORE_MIMIC: SpriteConfiguration(
        vulnerability=SpriteVulnerability.BOW, role=SpriteType.ENEMY
    ),
    SpriteId.x85_YELLOW_STALFOS: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x86_KODONGO: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x87_KODONGO_FIRE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x88_MOTHULA: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.x89_MOTHULA_BEAM: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.x8A_SPIKE_BLOCK: SpriteConfiguration(
        role=SpriteType.HAZARD, movement=SpriteMovement.HORIZONTAL
    ),
    SpriteId.x8B_GIBDO: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x8C_ARRGHUS: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.x8D_ARRGHUS_SPAWN: SpriteConfiguration(
        can_shuffle_in_room=False, role=SpriteType.ENEMY
    ),
    SpriteId.x8E_TERRORPIN: SpriteConfiguration(
        vulnerability=SpriteVulnerability.HAMMER, role=SpriteType.ENEMY
    ),
    SpriteId.x8F_BLOB: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x90_WALLMASTER: SpriteConfiguration(
        can_shuffle=False, role=SpriteType.OVERLORD
    ),
    SpriteId.x91_STALFOS_KNIGHT: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x92_KING_HELMASAUR: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.x93_BUMPER: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.FIXED,
    ),
    SpriteId.x94_PIROGUSU: SpriteConfiguration(is_aquatic=True, role=SpriteType.ENEMY),
    SpriteId.x95_EYE_LASER_EAST: SpriteConfiguration(
        role=SpriteType.HAZARD, movement=SpriteMovement.EAST
    ),
    SpriteId.x96_EYE_LASER_WEST: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.WEST,
    ),
    SpriteId.x97_EYE_LASER_SOUTH: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.SOUTH,
    ),
    SpriteId.x98_EYE_LASER_NORTH: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.NORTH,
    ),
    SpriteId.x99_PENGATOR: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x9A_KYAMERON: SpriteConfiguration(
        is_aquatic=True,
        role=SpriteType.ENEMY,
        vulnerability=SpriteVulnerability.INVULNERABLE,
    ),
    SpriteId.x9B_WIZZROBE: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.x9C_BABASU_SOUTH: SpriteConfiguration(
        can_hold_key=False,
        role=SpriteType.HAZARD,
        movement=SpriteMovement.SOUTH,
    ),
    SpriteId.x9D_BABUSU_EAST: SpriteConfiguration(
        can_hold_key=False,
        role=SpriteType.HAZARD,
        movement=SpriteMovement.EAST,
    ),
    SpriteId.x9E_HAUNTED_GROVE_OSTRICH: SpriteConfiguration(role=SpriteType.CREATURE),
    SpriteId.x9F_HAUNTED_GROVE_RABBIT: SpriteConfiguration(role=SpriteType.CREATURE),
    SpriteId.xA0_HAUNTED_GROVE_BIRD: SpriteConfiguration(role=SpriteType.CREATURE),
    SpriteId.xA1_FREEZOR: SpriteConfiguration(
        vulnerability=SpriteVulnerability.FIRE,
        can_shuffle_in_room=False,  # Collision box can block progress in Misery Mire.
        role=SpriteType.ENEMY,
    ),
    SpriteId.xA2_KHOLDSTARE: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xA3_KHOLDSTARES_SHELL: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xA4_FALLING_ICE: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.DIAGONAL,
    ),
    SpriteId.xA5_BLUE_ZAZAK: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xA6_RED_ZAZAK: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xA7_STALFOS: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xA8_GREEN_ZIRRO: SpriteConfiguration(
        can_hold_key=False,
        is_flying=True,
        role=SpriteType.ENEMY,
    ),
    SpriteId.xA9_BLUE_ZIRRO: SpriteConfiguration(
        can_hold_key=False,
        is_flying=True,
        role=SpriteType.ENEMY,
    ),
    SpriteId.xAA_PIKIT_LIKE_LIKE: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xAB_CRYSTAL_MAIDEN: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xAC_APPLE: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xAD_LOST_OLD_MAN: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xAE_DOWN_PIPE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xAF_UP_PIPE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xB0_RIGHT_PIPE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xB1_LEFT_PIPE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xB2_GOOD_BEE: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xB3_PEDESTAL_PLAQUE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xB4_PURPLE_CHEST: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xB5_BOMB_SALESMAN: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xB6_KIKI: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xB7_BLIND_MAIDEN: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xB8_DIALOGUE_TESTER: SpriteConfiguration(
        vulnerability=SpriteVulnerability.BOW,
        role=SpriteType.ENEMY,
    ),
    SpriteId.xB9_BULLY_AND_FRIEND: SpriteConfiguration(
        role=SpriteType.NPC
    ),
    SpriteId.xBA_WHIRLPOOL: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xBB_SALESMAN: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xBC_DRUNK_IN_THE_INN: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xBD_VITREOUS: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xBE_VITREOUS_SMALL_EYEBALL: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xBF_LIGHTNING: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xC0_CATFISH: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xC1_AGAHNIM_TELEPORTING: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xC2_BOULDER: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xC3_GIBO: SpriteConfiguration(role=SpriteType.ENEMY),
    # Note: This shows up as a "smurf" due to a wrong palette in lumberjack Overworld Area.
    SpriteId.xC4_THIEF: SpriteConfiguration(
        vulnerability=SpriteVulnerability.INVULNERABLE, role=SpriteType.ENEMY
    ),
    SpriteId.xC5_MEDUSA: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.FIXED,
    ),
    SpriteId.xC6_MEDUSA_FOUR_WAY: SpriteConfiguration(
        role=SpriteType.HAZARD,
        movement=SpriteMovement.FIXED,
    ),
    SpriteId.xC7_POKEY: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xC8_GREAT_FAIRY: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xC9_TEKTITE: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xCA_CHAIN_CHOMP: SpriteConfiguration(
        vulnerability=SpriteVulnerability.INVULNERABLE, role=SpriteType.ENEMY
    ),
    SpriteId.xCB_TRINEXX_ROCK: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xCC_TRINEXX_FIRE: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xCD_TRINEXX_ICE: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xCE_BLIND: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xCF_SWAMOLA: SpriteConfiguration(is_aquatic=True, role=SpriteType.ENEMY),
    SpriteId.xD0_LYNEL: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xD1_BUNNY_BEAM: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xD2_FLOPPING_FISH: SpriteConfiguration(role=SpriteType.CREATURE),
    SpriteId.xD3_STAL: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xD4_LANDMINE: SpriteConfiguration(role=SpriteType.ENEMY),
    SpriteId.xD5_DIGGING_GAME_PROPRIETOR: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xD6_GANON: SpriteConfiguration(role=SpriteType.BOSS),
    SpriteId.xD7_GANON_INVINCIBLE: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xD8_HEART: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xD9_GREEN_RUPEE: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xDA_BLUE_RUPEE: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xDB_RED_RUPEE: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xDC_BOMB_REFILL_1: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xDD_BOMB_REFILL_4: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xDE_BOMB_REFILL_8: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xDF_SMALL_MAGIC_REFILL: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xE0_FULL_MAGIC_REFILL: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xE1_ARROW_REFILL_5: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xE2_ARROW_REFILL_10: SpriteConfiguration(role=SpriteType.CONSUMABLE),
    SpriteId.xE3_FAIRY: SpriteConfiguration(
        can_shuffle_in_room=False, role=SpriteType.CONSUMABLE
    ),
    SpriteId.xE4_SMALL_KEY: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xE5_BIG_KEY: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xE6_SHIELD: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xE7_MUSHROOM: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xE8_FAKE_MASTER_SWORD: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xE9_MAGIC_MERCHANT: SpriteConfiguration(role=SpriteType.NPC),
    SpriteId.xEA_HEART_CONTAINER: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xEB_HEART_PIECE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xEC_THROWN_ITEM: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xED_SOMARIA_PLATFORM: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xEE_CASTLE_MANTLE: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xEF_SOMARIA_PLATFORM_UNUSED_1: SpriteConfiguration(
        role=SpriteType.OBJECT
    ),
    SpriteId.xF0_SOMARIA_PLATFORM_UNUSED_2: SpriteConfiguration(
        role=SpriteType.OBJECT
    ),
    SpriteId.xF1_SOMARIA_PLATFORM_UNUSED_3: SpriteConfiguration(
        role=SpriteType.OBJECT
    ),
    SpriteId.xF2_MEDALLION_TABLET: SpriteConfiguration(role=SpriteType.OBJECT),
    SpriteId.xF3_PERSONS_DOOR_OW_OVERLORD: SpriteConfiguration(
        role=SpriteType.OVERLORD
    ),
    SpriteId.xF4_FALLING_ROCKS_OW_OVERLORD: SpriteConfiguration(
        role=SpriteType.OVERLORD
    ),
    SpriteId.xF5_CANON_BALLS_OW_OVERLORD: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.xF6_UNKNOWN_F6_OW_OVERLORD: SpriteConfiguration(
        can_shuffle_in_room=False, role=SpriteType.OVERLORD
    ),
    SpriteId.xF7_UNKNOWN_F7_OW_OVERLORD: SpriteConfiguration(
        can_shuffle_in_room=False, role=SpriteType.OVERLORD
    ),
    SpriteId.xF8_UNKNOWN_F8_OW_OVERLORD: SpriteConfiguration(
        can_shuffle_in_room=False, role=SpriteType.OVERLORD
    ),
    SpriteId.xF9_UNKNOWN_F9_OW_OVERLORD: SpriteConfiguration(
        can_shuffle_in_room=False, role=SpriteType.OVERLORD
    ),
    SpriteId.xFA_BLOB_DROP_OW_OVERLORD: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.xFB_OVERWORLD_WALLMASTER_OW_OVERLORD_DROPS_IN_HOULIHAN: SpriteConfiguration(
        role=SpriteType.OVERLORD
    ),
    SpriteId.xFC_FLOOR_DROP_SQUARE_OW_OVERLORD: SpriteConfiguration(
        role=SpriteType.OVERLORD
    ),
    SpriteId.xFD_FLOOR_DROP_NORTH_PATH_OW_OVERLORD: SpriteConfiguration(
        role=SpriteType.OVERLORD
    ),
    SpriteId.xFE_FLOOR_DROP_EAST_PATH_OW_OVERLORD: SpriteConfiguration(
        role=SpriteType.OVERLORD
    ),
    SpriteId.x102_CANON_BALLS_EP_4_WALL_CANONBALLS: SpriteConfiguration(
        role=SpriteType.OVERLORD
    ),
    SpriteId.x103_CANON_BALLS_EP_ENTRY: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x106_BOMB_DROP1_TRAP: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x104_ROPE_DROP_TRAP: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x105_STALFOS_HEAD_TRAP: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x11A_BOMB_DROP2_TRAP: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x107_MOVING_FLOOR: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x108_TRANSFORMER_BUNNY_BEAM: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x109_WALLMASTER_OVERLORD: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x10A_FLOOR_DROP_SQUARE: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x10B_FLOOR_DROP_NORTH: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x110_PIROGUSU_SPAWNER_RIGHT: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x111_PIROGUSU_SPAWNER_LEFT: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x112_PIROGUSU_SPAWNER_DOWN: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x113_PIROGUSU_SPAWNER_UP: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x114_FLYING_FLOOR_TILE_TRAP: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x115_WIZZROBE_SPAWNER: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x116_ZORO_SPAWNER: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x117_FOUR_SKULL_TRAP_IN_POD_UNDER_POT: SpriteConfiguration(
        role=SpriteType.OVERLORD
    ),
    SpriteId.x118_STALFOS_APPEAR: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x119_ARMOS_KNIGHTS_TRIGGER: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x11A_BOMB_DROP2_TRAP: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x141_SOLDIER_ALERTER_BLUE: SpriteConfiguration(role=SpriteType.OVERLORD),
    SpriteId.x142_SOLDIER_ALERTER_GREEN: SpriteConfiguration(role=SpriteType.OVERLORD),
}
