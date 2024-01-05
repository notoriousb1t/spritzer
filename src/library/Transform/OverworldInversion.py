from typing import Dict, List, Tuple
from ..Model import OverworldArea, OverworldAreaId, SpriteId, SpriteSheetId, SpritesetId
from .Context import Context


# List of special areas that require preservation of some of their sheets.
# If all indexes are preserved, the sprite sheet is left fully intact.
# If a non-zero number of indexes less than max are provided, it creates a new spritesheet to capture changes.
_SPECIAL_AREAS: Dict[OverworldAreaId, Tuple[List[int], List[int], List[int]]] = {
    # Preserve Witch and zoras
    OverworldAreaId.x16_WITCHS_HUT: ([2], [2]),
    # This is only NPCs, so just skip inversion.
    OverworldAreaId.x28_KAKARIKO_VILLAGE_MAZE_RACE: (range(4), range(4)),
    # Preserve frog black smith
    OverworldAreaId.x29_KAKARIKO_VILLAGE_LIBRARY: ([], range(4)),
    # Preserve all existing NPCs
    OverworldAreaId.x18_KAKARIKO_VILLAGE: (range(4), []),
    # Purple chest.
    OverworldAreaId.x22_SMITHY: ([], range(4)),
    # Only NPCs and creatures.
    OverworldAreaId.x2A_HAUNTED_GROVE: (range(4), range(4)),
    # Don't bother inverting master sword area or man camping under bridge.
    OverworldAreaId.x40_MASTER_SWORD_UNDER_BRIDGE: (range(4), range(4)),
    # Lumberjack in lw_v1 and dash item in lw_v2
    OverworldAreaId.x2_LUMBER_JACK_HOUSE: (range(4), []),
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
        target = SpriteId.x19_POE
        if area.lw_v1 and target in [sprite.sprite_id for sprite in area.lw_v1.sprites]:
            print(f"{area.id}")
            print(f" - Set {area.lw_v1.spriteset_id}")
            print(f" - Sheet {context.spritesets[area.lw_v1.spriteset_id].sheet3}")
        if area.lw_v2 and target in [sprite.sprite_id for sprite in area.lw_v2.sprites]:
            print(f"{area.id}")
            print(f" - Set {area.lw_v2.spriteset_id}")
            print(f" - Sheet {context.spritesets[area.lw_v2.spriteset_id].sheet3}")
        if area.dw and target in [sprite.sprite_id for sprite in area.dw.sprites]:
            print(f"{area.id}")
            print(f" - Set {area.dw.spriteset_id}")
            print(f" - Sheet {context.spritesets[area.dw.spriteset_id].sheet3}")

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
            if sprite.sprite_id == SpriteId.xD2_FLOPPING_FISH:
                # This generally works everywhere.
                sprite.sprite_id = SpriteId.xD3_STAL

    for area in context.overworld_areas.values():
        if area.id in _SPECIAL_AREAS:
            _invert_special_overworld_versions(context, area)
    for area in context.overworld_areas.values():
        if not area.id in _SPECIAL_AREAS:
            _invert_normal_overworld_versions(area)

    # Some manual tweaks.
    
    # Restores Poes in Dark Kakariko.
    thieves_village_spriteset = context.spritesets[SpritesetId.x15_THIEVES_VILLAGE]
    thieves_village_spriteset.sheet3 = SpriteSheetId.x15_THIEF_DW
