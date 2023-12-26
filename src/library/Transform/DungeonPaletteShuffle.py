from typing import Dict, List

from ..Model.PaletteId import PaletteId

from ..Model import DungeonRoom, DungeonRoomId, SpriteType
from .Context import Context

_cool_dungeons = [
    PaletteId.x04_ICE_DUNGEON,
    PaletteId.x06_TOWER_OF_HERA,
    PaletteId.x08_SWAMP_PALACE,
    PaletteId.x0A_SWAMP_PALACE,
    PaletteId.x0C_AGAHNIMS_TOWER,
    PaletteId.x0D_SKULL_WOODS,
    PaletteId.x13_ICE_PALACE,
    PaletteId.x14_ICE_PALACE_KHOLDSTARE,
    PaletteId.x18_TURTLE_ROCK,
    PaletteId.x19_TURTLE_ROCK_TRINEXX,
    PaletteId.x23_BLIND,
    PaletteId.x26_AGAHNIMS_TOWER,
]
_neutral_dungeons = [
    PaletteId.x05_DESERT_DUNGEON,
    PaletteId.x09_DESERT_PALACE,
    PaletteId.x0B_MISERY_MIRE,
    PaletteId.x0F_PALACE_OF_DARKNESS,
    PaletteId.x10_PALACE_OF_DARKNESS_HELMASAUR,
    PaletteId.x11_MISERY_MIRE,
    PaletteId.x12_MISERY_MIRE_VITREUS,
    PaletteId.x12_MISERY_MIRE_VITREUS,
    PaletteId.x1A_GANONS_TOWER,
    PaletteId.x1B_GANONS_TOWER,
    PaletteId.x21_GANON,
    PaletteId.x24_GANONS_TOWER,
    PaletteId.x25_GANONS_TOWER,
    PaletteId.x28_GANONS_TOWER,
]
_warm_dungeons = [
    PaletteId.x00_HYRULE_CASTLE,
    PaletteId.x03_GREEN_DUNGEON,
    PaletteId.x05_DESERT_DUNGEON,
    PaletteId.x09_DESERT_PALACE,
]

_palette_lists = [
    _cool_dungeons,
    _neutral_dungeons,
    _warm_dungeons,
]


def reroll_dungeon_palette(context: Context) -> None:
    palette_dict: Dict[PaletteId, List[DungeonRoom]] = {
        it: list() for it in list(PaletteId)
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
            if any(
                it
                for it in room.dungeon_sprites
                if it.sprite_id.meta().role in [SpriteType.BOSS, SpriteType.NPC]
            ):
                # Boss randomizer should handle palette swapping for boss rooms.
                continue
            room.palette_id = new_palette_id
