from enum import IntEnum


class DungeonRoomTilesetId(IntEnum):
    """The resource id of walls, floors, etc. of the Dungeon Room."""

    x0_CASTLE = 0
    x1_DUNGEON = 1
    x2_AGAHNIM = 2
    x3_HOUSE = 3
    x4_SANTUARY = 4
    x5_EASTERN_PALACE = 5
    x6_TOWER1 = 6
    x7_TOWER_HERA = 7
    x8_SWAMP_PALACE = 8
    x9_SKULL_WOODS = 9
    xA_THIEVES_TOWN = 10
    xB_ICE_PALACE = 11
    xC_MISERY_MIRE = 12
    xD_TURTLE_ROCK = 13
    xE_DESERT_PALACE = 14
    xF_SAHASRAHLA = 15
    x10_LINKS_HOUSE = 16
    x11_SHOP = 17
    x12_FAIRY_CAVE = 18
    x13_GANON = 19
    x14_CAVE = 20

    def __str__(self) -> str:
        return self.name
