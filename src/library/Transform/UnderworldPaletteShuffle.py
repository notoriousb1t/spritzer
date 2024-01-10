from typing import Dict, List

from library.Model.PaletteId import PaletteId
from library.Model.SpriteType import SpriteType
from library.Model.UnderworldRoom import UnderworldRoom
from library.Transform.Context import Context

_underworld_palettes = [
    PaletteId.x00_HYRULE_CASTLE,
    PaletteId.x03_GREEN_DUNGEON,
    PaletteId.x04_ICE_DUNGEON,
    PaletteId.x04_ICE_DUNGEON,  # Weight Ice Dungeons higher.
    PaletteId.x05_DESERT_DUNGEON,
    PaletteId.x06_TOWER_OF_HERA,
    PaletteId.x08_SWAMP_PALACE,
    PaletteId.x09_DESERT_PALACE,
    PaletteId.x09_DESERT_PALACE,  # Weight Desert higher.
    PaletteId.x0A_SWAMP_PALACE,
    PaletteId.x0B_MISERY_MIRE,
    PaletteId.x0C_AGAHNIMS_TOWER,
    PaletteId.x0D_SKULL_WOODS,
    PaletteId.x0F_PALACE_OF_DARKNESS,
    PaletteId.x0F_PALACE_OF_DARKNESS,  # Weight POD higher.
    PaletteId.x11_MISERY_MIRE,
    PaletteId.x13_ICE_PALACE,
    PaletteId.x18_TURTLE_ROCK,
    PaletteId.x1A_GANONS_TOWER,
    PaletteId.x1B_GANONS_TOWER,
    PaletteId.x21_GANON,
    PaletteId.x24_GANONS_TOWER,
    PaletteId.x25_GANONS_TOWER,
    PaletteId.x28_GANONS_TOWER,
]


def reroll_underworld_palette(context: Context) -> None:
    palette_dict: Dict[PaletteId, List[UnderworldRoom]] = {
        it: list() for it in list(PaletteId)
    }
    for room in context.underworld_rooms.values():
        palette_dict[room.palette_id].append(room)

    for room_list in palette_dict.values():
        if len(room_list) == 0:
            continue

        new_palette_id = context.random.choice(_underworld_palettes)
        for room in room_list:
            if any(
                it
                for it in room.sprites
                if it.sprite_id.meta().role in [SpriteType.BOSS, SpriteType.NPC]
            ):
                # Boss randomizer should handle palette swapping for boss rooms.
                continue
            room.palette_id = new_palette_id
