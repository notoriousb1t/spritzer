from typing import Dict, List, Tuple

from library.model.overworld_area import OverworldArea
from library.model.overworld_area_id import OverworldAreaId
from library.model.sprite_id import SpriteId
from library.model.sprite_sheet_id import SpriteSheetId
from library.model.spriteset import Spriteset
from library.model.spriteset_id import SpritesetId
from library.model.model import Model


# List of special areas that require preservation of some of their sheets.
# If all indexes are preserved, the sprite sheet is left fully intact.
# If a non-zero number of indexes less than max are provided, it creates a new spritesheet to capture changes.
_SPECIAL_AREAS: Dict[OverworldAreaId, Tuple[List[int], List[int]]] = {
    # Preserve Witch and zoras
    OverworldAreaId.x16_WITCHS_HUT: ([2], [2]),
    # This is only NPCs, so just skip inversion.
    OverworldAreaId.x28_KAKARIKO_VILLAGE_MAZE_RACE: (list(range(4)), list(range(4))),
    # Preserve frog black smith
    OverworldAreaId.x29_KAKARIKO_VILLAGE_LIBRARY: ([], list(range(4))),
    # Preserve all existing NPCs
    OverworldAreaId.x18_KAKARIKO_VILLAGE: (list(range(4)), []),
    # Purple chest.
    OverworldAreaId.x22_SMITHY: ([], [0, 3]),
    # Only NPCs and creatures.
    OverworldAreaId.x2A_HAUNTED_GROVE: (list(range(4)), list(range(4))),
    # Don't bother inverting master sword area or man camping under bridge.
    OverworldAreaId.x40_MASTER_SWORD_UNDER_BRIDGE: (list(range(4)), list(range(4))),
    # Lumberjack in lw_v1 and dash item in lw_v2
    OverworldAreaId.x2_LUMBER_JACK_HOUSE: (list(range(4)), []),
    # Preserve catfish.
    OverworldAreaId.xF_ENTRANCE_TO_ZORAS_DOMAIN: ([], [2, 3]),
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


def _invert_special_overworld_versions(context: Model, area: OverworldArea) -> None:
    if not area.dw or not area.lw_v2:
        # Skip inversion for areas that don't have both a light and dark world.
        # It is assumed that light world v2 exists if any light world exists
        return

    """Specific patches to preserve NPCs and objects, but non-restrictive overall."""
    (preserved_lw_sheets, preserved_dw_sheets) = _SPECIAL_AREAS[area.id]
    spriteset_lw_id: SpritesetId = (
        area.lw_v1.spriteset_id if area.lw_v1 else area.lw_v2.spriteset_id
    )
    spriteset_lw_palette: int = area.lw_v2.sprite_palette_id
    spriteset_lw: Spriteset = context.spritesets[spriteset_lw_id]
    spriteset_dw_id: SpritesetId = area.dw.spriteset_id
    spriteset_dw_palette: int = area.dw.sprite_palette_id
    spriteset_dw: Spriteset = context.spritesets[spriteset_dw_id]

    if len(preserved_lw_sheets) == 0:
        if area.lw_v1:
            area.lw_v1.spriteset_id = spriteset_dw_id
            area.lw_v1.sprite_palette_id = spriteset_dw_palette
        area.lw_v2.spriteset_id = spriteset_dw_id
        area.lw_v2.sprite_palette_id = spriteset_dw_palette
    elif len(preserved_lw_sheets) == 4:
        # If we are preserving all 4 spots, do nothing.
        pass
    else:
        spriteset_id: SpritesetId = context.unused_spritesets.pop()
        spriteset: Spriteset = context.spritesets[spriteset_id]

        if area.lw_v1:
            area.lw_v1.spriteset_id = spriteset_id
            area.lw_v1.sprite_palette_id = spriteset_dw_palette
        area.lw_v2.spriteset_id = spriteset_id
        area.lw_v2.sprite_palette_id = spriteset_dw_palette

        for index in list(range(4)):
            if index in preserved_lw_sheets:
                lw_spritesheet: SpriteSheetId | None = spriteset_lw.get_at(index=index)
                if lw_spritesheet != None:
                    spriteset.set_at(index=index, value=lw_spritesheet)
            else:
                dw_spritesheet: SpriteSheetId | None = spriteset_dw.get_at(index=index)
                if dw_spritesheet != None:
                    spriteset.set_at(index=index, value=dw_spritesheet)

    if len(preserved_dw_sheets) == 0:
        area.dw.spriteset_id = spriteset_lw_id
        area.dw.sprite_palette_id = spriteset_lw_palette
    elif len(preserved_dw_sheets) == 4:
        # If we are preserving all 4 spots, do nothing.
        pass
    else:
        # Free a spriteset up and reassign this version to that spriteset.
        spriteset_id = context.unused_spritesets.pop()
        spriteset = context.spritesets[spriteset_id]
        area.dw.spriteset_id = spriteset_id
        area.dw.sprite_palette_id = spriteset_lw_palette

        for index in list(range(4)):
            if index in preserved_dw_sheets:
                dw_spritesheet: SpriteSheetId | None = spriteset_dw.get_at(index=index)
                if dw_spritesheet != None:
                    spriteset.set_at(index=index, value=dw_spritesheet)
            else:
                lw_spritesheet = spriteset_lw.get_at(index=index)
                if lw_spritesheet != None:
                    spriteset.set_at(index=index, value=lw_spritesheet)


def _invert_normal_overworld_versions(area: OverworldArea) -> None:
    if not area.dw or not area.lw_v2:
        # Skip inversion for areas that don't have both a light and dark world.
        # It is assumed that light world v2 exists if any light world exists
        return

    # Capture palette and spritesheet initial values upfront
    light_sprite_palette_id: int = area.lw_v2.sprite_palette_id
    light_spriteset_id: SpritesetId = area.lw_v2.spriteset_id
    dark_sprite_palette_id: int = area.dw.sprite_palette_id
    dark_spriteset_id: SpritesetId = area.dw.spriteset_id

    if area.lw_v1:
        area.lw_v1.sprite_palette_id = dark_sprite_palette_id
        area.lw_v1.spriteset_id = dark_spriteset_id
    area.lw_v2.sprite_palette_id = dark_sprite_palette_id
    area.lw_v2.spriteset_id = dark_spriteset_id
    area.dw.sprite_palette_id = light_sprite_palette_id
    area.dw.spriteset_id = light_spriteset_id


def invert_world(context: Model) -> None:
    # Replace talking trees with fairies. This serves 2 purposes:
    # - introduces more consumables to dark world
    # - generalizes areas because Talking Trees are considered NPCs.
    # - Replaces problematic creatures.
    for area in context.overworld_areas.values():
        for sprite in area.all_sprites:
            if sprite.sprite_id == SpriteId.x25_TALKING_TREE:
                # Turns talking trees into bonkable trees.
                sprite.sprite_id = SpriteId.xE3_FAIRY
            if sprite.sprite_id == SpriteId.x4D_TOPPO:
                # This generally works everywhere.
                sprite.sprite_id = SpriteId.xD3_STAL

    for area in context.overworld_areas.values():
        if area.id in _SPECIAL_AREAS:
            _invert_special_overworld_versions(context=context, area=area)
    for area in context.overworld_areas.values():
        if not area.id in _SPECIAL_AREAS:
            _invert_normal_overworld_versions(area=area)

    # Some manual tweaks.
    # This swap doesn't work as well because kakariko has no flying creatures.
    kakariko_vilage: OverworldArea = context.overworld_areas[
        OverworldAreaId.x18_KAKARIKO_VILLAGE
    ]
    kakariko_path: OverworldArea = context.overworld_areas[
        OverworldAreaId.x10_PATH_BETWEEN_KAKARIKO_VILLAGE_AND_LOST_WOODS
    ]
    for sprites in [
        kakariko_path.dw.sprites if kakariko_path.dw else [],
        kakariko_vilage.dw.sprites if kakariko_vilage.dw else [],
    ]:
        for sprite in sprites:
            if sprite.sprite_id == SpriteId.x19_POE:
                sprite.sprite_id = SpriteId.xB_CUCCO

    # Restores Poes in Dark Kakariko.
    thieves_village_spriteset: Spriteset = context.spritesets[
        SpritesetId.x15_THIEVES_VILLAGE
    ]
    thieves_village_spriteset.sheet3 = SpriteSheetId.x15_THIEF_DW
