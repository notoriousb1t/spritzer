from attr import dataclass
from enum import IntEnum

_room_gfx_offset = 64


class SpriteBlocksetId(IntEnum):
    # Overworld Area Graphics Range 0-64
    x00_PATH = 0
    x01_HYRULE_CASTLE_FOREST = 1
    x02_BRIDGE = 2
    x03_GRAVEYARD = 3
    x04_LAKE_HYLIA = 4
    x05_RUINS = 5
    x06_KAKARIKO = 6
    x07_LOST_WOODS = 7
    x08_DESERT = 8
    x09_KAKARIKO_LIBRARY = 9
    x0A_PATH = 10
    x0B_PATH = 11
    x0C_BRIDGE = 12
    x0D_WITCH = 13
    x0E_ZORAS_DOMAIN = 14
    x0F_HAUNTED_GROVE = 15
    x10_DEATH_MOUNTAIN = 16
    x11_HAUNTED_GROVE = 17
    x12_POND = 18
    x13_SKULL_WOODS = 19
    x14_DEATHMOUNTAIN = 20
    x15_THIEVES_VILLAGE = 21
    x16_RUINS = 22
    x17_RUINS = 23
    x18_PATH = 24
    x19_LAKE = 25
    x1A_PATH = 26
    xAB_DIGGING_GAME = 27
    x1C_HYRULE_CASTLE = 28
    x1D_THIEVES_VILLAGE = 29
    x1E_UNUSED = 30
    x1F_DESERT = 31
    x20_SWAMP = 32
    x21_UNUSED = 33
    x22_UNUSED = 34
    x23_PATH = 35
    x24_UNUSED = 36
    x25_UNUSED = 37
    x26_PYRAMID = 38
    # Dungeon Room Graphics Range 65-?
    x01_SEWER_AND_CAVES = 1 + _room_gfx_offset
    x02_FIRST_DARK_ROOM = 2 + _room_gfx_offset
    x03_THRONE_ROOM = 3 + _room_gfx_offset
    x04_HYRULE_CASTLE = 4 + _room_gfx_offset
    x05_SHOP = 5 + _room_gfx_offset
    x06_SANCTUARY = 6 + _room_gfx_offset
    x07_FAIRY_CAVE = 7 + _room_gfx_offset
    x08_EASTERN_PALACE = 8 + _room_gfx_offset
    x09_ARMOS = 9 + _room_gfx_offset
    x0A_DESERT_PALACE = 10 + _room_gfx_offset
    x0B_LANMOLAS = 11 + _room_gfx_offset
    x0C_MOLDORM = 12 + _room_gfx_offset
    x0D_LINKS_HOUSE = 13 + _room_gfx_offset
    x0E_CHICKEN_HOUSE = 14 + _room_gfx_offset
    x0F_HOUSE = 15 + _room_gfx_offset
    x10_SAHASRAHLA = 16 + _room_gfx_offset
    x11_SWAMP_PALACE = 17 + _room_gfx_offset
    x12_MAIDENS_CHAMBER = 18 + _room_gfx_offset
    x13_SKULL_WOODS_ROOM = 19 + _room_gfx_offset
    x14_ARRGHUS = 20 + _room_gfx_offset
    x15_HELMASAUR = 21 + _room_gfx_offset
    x16_VITREOUS_KHOLDSTARE = 22 + _room_gfx_offset
    x17_TURTLE_ROCK = 23 + _room_gfx_offset
    x18_AGAHNIM2 = 24 + _room_gfx_offset
    x19_PALACE_OF_DARKNESS = 25 + _room_gfx_offset
    x1A_MOTHULA = 26 + _room_gfx_offset
    x1B_BLINDS_HIDEOUT = 27 + _room_gfx_offset
    x1C_ICE_PALACE = 28 + _room_gfx_offset
    x1D_MISERY_MIRE = 29 + _room_gfx_offset
    x1E_TURTLE_ROCK2 = 30 + _room_gfx_offset
    x1F_UNUSED = 31 + _room_gfx_offset
    x20_BLIND = 32 + _room_gfx_offset
    x21_AGAHNIMS_TOWER = 33 + _room_gfx_offset
    x22_GANON = 34 + _room_gfx_offset
    x23_GANONS_TOWER1 = 35 + _room_gfx_offset
    x24_GANONS_TOWER2 = 36 + _room_gfx_offset
    x25_TURTLE_ROCK3 = 37 + _room_gfx_offset
    x26_TURTLE_ROCK4 = 38 + _room_gfx_offset
    x27_AGAHNIMS_TOWER_BRIDGE = 39 + _room_gfx_offset
    x28_CHECKERBOARD_CAVE = 40 + _room_gfx_offset
    X29_ICE_PALACE2 = 41 + _room_gfx_offset
    x2A_HYPE_CAVE = 42 + _room_gfx_offset
    x2B_PALACE_OF_DARKNESS2 = 43 + _room_gfx_offset

    def __str__(self) -> str:
        return self.name

    def get_room_value(self) -> int:
        return self.value - _room_gfx_offset

    @staticmethod
    def from_room_value(value: int):
        return SpriteBlocksetId(value + _room_gfx_offset)


@dataclass
class SpriteBlockset:
    """Loads the a blockset of sprites. This contains up to 4 sprite sets that load in individual sprites"""

    id: SpriteBlocksetId
    set0: int = None
    set1: int = None
    set2: int = None
    set3: int = None
