from typing import Dict, List

from ..Model import DungeonPaletteId, DungeonRoom
from .Context import Context


_caves = [
    DungeonPaletteId.x01_BLUE_CAVES,
    DungeonPaletteId.x07_CAVE,
    DungeonPaletteId.x20_CAVE,
    DungeonPaletteId.x22_GREAT_FAIRY,
    DungeonPaletteId.x27_MIMICS_CAVE,
]
_cool_dungeons = [
    DungeonPaletteId.x04_ICE_DUNGEON,
    DungeonPaletteId.x06_TOWER_OF_HERA,
    DungeonPaletteId.x08_SWAMP_PALACE,
    DungeonPaletteId.x0A_SWAMP_PALACE,
    DungeonPaletteId.x0C_AGAHNIMS_TOWER,
    DungeonPaletteId.x0D_SKULL_WOODS,
    DungeonPaletteId.x13_ICE_PALACE,
    DungeonPaletteId.x14_ICE_PALACE_KHOLDSTARE,
    DungeonPaletteId.x18_TURTLE_ROCK,
    DungeonPaletteId.x19_TURTLE_ROCK_TRINEXX,
    DungeonPaletteId.x23_BLIND,
    DungeonPaletteId.x26_AGAHNIMS_TOWER,
]
_neutral_dungeons = [
    DungeonPaletteId.x05_DESERT_DUNGEON,
    DungeonPaletteId.x09_DESERT_PALACE,
    DungeonPaletteId.x0B_MISERY_MIRE,
    DungeonPaletteId.x0F_PALACE_OF_DARKNESS,
    DungeonPaletteId.x10_PALACE_OF_DARKNESS_HELMASAUR,
    DungeonPaletteId.x11_MISERY_MIRE,
    DungeonPaletteId.x12_MISERY_MIRE_VITREUS,
    DungeonPaletteId.x12_MISERY_MIRE_VITREUS,
    DungeonPaletteId.x1A_GANONS_TOWER,
    DungeonPaletteId.x1B_GANONS_TOWER,
    DungeonPaletteId.x21_GANON,
    DungeonPaletteId.x24_GANONS_TOWER,
    DungeonPaletteId.x25_GANONS_TOWER,
    DungeonPaletteId.x28_GANONS_TOWER,
]
_warm_dungeons = [
    DungeonPaletteId.x00_HYRULE_CASTLE,
    DungeonPaletteId.x03_GREEN_DUNGEON,
    DungeonPaletteId.x05_DESERT_DUNGEON,
    DungeonPaletteId.x09_DESERT_PALACE,
]

_houses_and_shops = [
    DungeonPaletteId.x02_HOUSES,
    DungeonPaletteId.x15_LINKS_HOUSE,
    DungeonPaletteId.x17_THIEVES_TOWN,
    DungeonPaletteId.x1C_SAHASRAHLAS_HOUSE,
    DungeonPaletteId.x1E_SHOP,
    DungeonPaletteId.x1F_SHOP,
]

_palette_lists = [
    _caves,
    _cool_dungeons,
    _houses_and_shops,
    _neutral_dungeons,
    _warm_dungeons,
]


def reroll_dungeon_palette(context: Context) -> None:
    palette_dict: Dict[DungeonPaletteId, List[DungeonRoom]] = {
        it: list() for it in list(DungeonPaletteId)
    }
    for room in context.dungeon_rooms.values():
        palette_dict[room.palette_id].append(room)

    for palette_id, room_list in palette_dict.items():
        if len(room_list) == 0:
            continue

        matching_palettes = next(
            (it for it in _palette_lists if palette_id in it), None
        )
        if matching_palettes == None:
            continue

        new_palette_id = context.random.choice(matching_palettes)
        for room in room_list:
            room.palette_id = new_palette_id
