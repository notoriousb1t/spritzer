from typing import Dict, List, Tuple
from ..Model import OverworldArea, OverworldAreaId, SpriteSheetId
from . import Context


# List of special areas that require preservation of some of their sheets.
# If all indexes are preserved, the sprite sheet is left fully intact.
# If a non-zero number of indexes less than max are provided, it creates a new spritesheet to capture changes.
_SPECIAL_AREAS: Dict[OverworldAreaId, Tuple[List[int], List[int], List[int]]] = {
    OverworldAreaId.x16_WITCHS_HUT: ([2], []),
    OverworldAreaId.x28_KAKARIKO_VILLAGE_MAZE_RACE: (range(4), range(4)),
    OverworldAreaId.x18_KAKARIKO_VILLAGE: (range(4), []),
    OverworldAreaId.x2A_HAUNTED_GROVE: (range(4), range(4)),
    OverworldAreaId.x40_MASTER_SWORD_UNDER_BRIDGE: (range(4), range(4)),
    OverworldAreaId.x2_LUMBER_JACK_HOUSE: (range(4), []),
    OverworldAreaId.xF_ENTRANCE_TO_ZORAS_DOMAIN: ([], range(4)),
    # Hylia Obstacle and Tablet. Also preserve Fireball Zoras.
    OverworldAreaId.x30_DESERT_OF_MYSTERY: ([2], [2]),
    # Fake Sword
    OverworldAreaId.x0_LOST_WOODS: ([3], []),
    # Rocks, Friends
    OverworldAreaId.x3_WEST_DEATH_MOUNTAIN: ([2, 3], [3]),
    # Electric Barrier / Lighting Lock
    OverworldAreaId.x1B_HYRULE_CASTLE: ([3], []),
    # Kiki
    OverworldAreaId.x1E_EASTERN_PALACE: ([], [3]),
    # Average middle aged man.
    OverworldAreaId.x3A_PATH_BETWEEN_DESERT_OF_MYSTERY_AND_GREAT_SWAMP: ([3], []),
    # Toppo
    OverworldAreaId.x3B_SOUTH_WESTERN_GREAT_SWAMP: ([3], []),
    # Toppo
    OverworldAreaId.x34_NORTH_EASTERN_GREAT_SWAMP: ([3], []),
}


def _invert_special_overworld_versions(context: Context, area: OverworldArea) -> None:
    if not area.dw or not area.lw_v2:
        # Skip inversion for areas that don't have both a light and dark world.
        # It is assumed that light world v2 exists if any light world exists
        return

    """Specific patches to preserve NPCs and objects, but non-restrictive overall."""
    (preserved_lw_sheets, preserved_dw_sheets) = _SPECIAL_AREAS[area.id]
    spriteset_lw_id = area.lw_v1.spriteset_id if area.lw_v1 else area.lw_v2.spriteset_id
    spriteset_lw = context.spritesets[spriteset_lw_id]
    spriteset_dw_id = area.dw.spriteset_id
    spriteset_dw = context.spritesets[spriteset_dw_id]

    if len(preserved_lw_sheets) == 0:
        area.lw_v1.spriteset_id = spriteset_dw_id
        area.lw_v2.spriteset_id = spriteset_dw_id
    elif len(preserved_lw_sheets) == 4:
        # If we are preserving all 4 spots, do nothing.
        pass
    else:
        spriteset_id = context.unused_spritesets.pop()
        spriteset = context.spritesets[spriteset_id]

        area.lw_v1.spriteset_id = spriteset_id
        area.lw_v2.spriteset_id = spriteset_id

        for index in range(4):
            if index in preserved_lw_sheets:
                spriteset.set_at(index, spriteset_lw.get_at(index))
            else:
                spriteset.set_at(index, spriteset_dw.get_at(index))

    if len(preserved_dw_sheets) == 0:
        area.dw.spriteset_id = spriteset_lw_id
    elif len(preserved_dw_sheets) == 4:
        # If we are preserving all 4 spots, do nothing.
        pass
    else:
        # Free a spriteset up and reassign this version to that spriteset.
        spriteset_id = context.unused_spritesets.pop()
        spriteset = context.spritesets[spriteset_id]
        area.dw.spriteset_id = spriteset_id

        for index in range(4):
            if index in preserved_dw_sheets:
                spriteset.set_at(index, spriteset_dw.get_at(index))
            else:
                spriteset.set_at(index, spriteset_lw.get_at(index))


def _invert_normal_overworld_versions(area: OverworldArea) -> None:
    if not area.dw or not area.lw_v2:
        # Skip inversion for areas that don't have both a light and dark world.
        # It is assumed that light world v2 exists if any light world exists
        return

    # Capture palette and spritesheet initial values upfront
    light_sprite_palette_id = area.lw_v2.sprite_palette_id
    light_spriteset_id = area.lw_v2.spriteset_id
    dark_sprite_palette_id = area.dw.sprite_palette_id
    dark_spriteset_id = area.dw.spriteset_id

    area.lw_v1.sprite_palette_id = dark_sprite_palette_id
    area.lw_v1.spriteset_id = dark_spriteset_id
    area.lw_v2.sprite_palette_id = dark_sprite_palette_id
    area.lw_v2.spriteset_id = dark_spriteset_id
    area.dw.sprite_palette_id = light_sprite_palette_id
    area.dw.spriteset_id = light_spriteset_id


def invert_world(context: Context) -> None:
    for area in context.overworld_areas.values():
        if area.id in _SPECIAL_AREAS:
            _invert_special_overworld_versions(context, area)
    for area in context.overworld_areas.values():
        if not area.id in _SPECIAL_AREAS:
            _invert_normal_overworld_versions(area)
