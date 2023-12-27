from enum import IntEnum


class DungeonRoomFloorId(IntEnum):
    """The floors of dungeons"""

    x00_GROUND_NEUTRAL = 0
    x01_GROUND_PRIMARY = 1
    x02_WOOD_PRIMARY = 2
    x03_WOOD_NEUTRAL = 3
    x04_GROUND_ACCENT = 4
    x05_GROUND_NEUTRAL = 5
    x06_WOOD_PRIMARY = 6
    x07_SIDE_TILE = 7
    x08_DARKNESS = 8
    x09_MOSAIC_PRIMARY = 9
    x0A_PEBBLES = 10
    x0B_WATER = 11
    x0C_DASHES = 12
    x0D_TILE = 13
    x0E_DARKNESS = 14
    x0F_DARKNESS = 15
    x10_DARKNESS = 16
    x11_DARKNESS = 17
    x12_DARKNESS = 18
    x13_DARKNESS = 19
    x14_DARKNESS = 20
    x15_DARKNESS = 21
    x16_DARKNESS = 22
    x17_DARKNESS = 23
    x18_DARKNESS = 24
    x19_DARKNESS = 25
    x1A_DARKNESS = 26
    x1B_DARKNESS = 27
    x1C_DARKNESS = 28
    x1D_DARKNESS = 29
    x1E_DARKNESS = 30
    x1F_DARKNESS = 31
    x20_DARKNESS = 32
    x21_DARKNESS = 33
    x22_DARKNESS = 34
    x23_DARKNESS = 35
    x25_DARKNESS = 37
    x26_DARKNESS = 38
    x27_DARKNESS = 39
    x28_DARKNESS = 40
    x40_DARKNESS = 64
    x60_DARKNESS = 96
    x80_DARKNESS = 128
    xC0_DARKNESS = 192
    xE0_DARKNESS = 224
    xFF_DARKNESS = 255

    def __str__(self) -> str:
        return self.name
