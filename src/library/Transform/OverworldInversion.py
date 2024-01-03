from typing import List
from ..Model import (
    OverworldArea,
    OverworldAreaId,
    SpriteSheetId,
)
from . import Context


def _apply_area_specific_tweaks(context: Context, area: OverworldArea) -> None:
    """Specific patches to preserve NPCs and objects, but non-restrictive overall."""
    if area.id == OverworldAreaId.x30_DESERT_OF_MYSTERY:
        misery_mire = context.spritesets[area.dark_world.spriteset_id]
        desert1 = context.spritesets[area.light_world_v1.spriteset_id]
        desert2 = context.spritesets[area.light_world_v2.spriteset_id]
        desert1.sheet2 = misery_mire.sheet2
        desert2.sheet2 = misery_mire.sheet2
    elif area.id == OverworldAreaId.x0_LOST_WOODS:
        skull_woods = context.spritesets[area.dark_world.spriteset_id]
        lost_woods1 = context.spritesets[area.light_world_v1.spriteset_id]
        lost_woods2 = context.spritesets[area.light_world_v2.spriteset_id]
        lost_woods1.sheet2 = skull_woods.sheet2
        lost_woods2.sheet2 = skull_woods.sheet2
    elif area.id == OverworldAreaId.x3_WEST_DEATH_MOUNTAIN:
        death_mountain_dw = context.spritesets[area.dark_world.spriteset_id]
        death_mountain1 = context.spritesets[area.light_world_v1.spriteset_id]
        death_mountain2 = context.spritesets[area.light_world_v2.spriteset_id]
        death_mountain1.sheet2 = death_mountain_dw.sheet2
        death_mountain1.sheet3 = death_mountain_dw.sheet3
        death_mountain2.sheet2 = death_mountain_dw.sheet2
        death_mountain2.sheet3 = death_mountain_dw.sheet3
        death_mountain_dw.sheet3 = SpriteSheetId.x14_FRIENDLY_LYNEL
    elif area.id == OverworldAreaId.x1E_EASTERN_PALACE:
        eastern_palace = context.spritesets[area.light_world_v2.spriteset_id]
        palace_of_darkness = context.spritesets[area.dark_world.spriteset_id]
        palace_of_darkness.sheet3 = eastern_palace.sheet3
    elif area.id == OverworldAreaId.x3A_PATH_BETWEEN_DESERT_OF_MYSTERY_AND_GREAT_SWAMP:
        path1 = context.spritesets[area.light_world_v1.spriteset_id]
        path2 = context.spritesets[area.light_world_v2.spriteset_id]
        path1.sheet3 = SpriteSheetId.x11_MISC_FAKE_SWORD
        path2.sheet3 = SpriteSheetId.x11_MISC_FAKE_SWORD
    elif area.id in [
        OverworldAreaId.x3B_SOUTH_WESTERN_GREAT_SWAMP,
        OverworldAreaId.x34_NORTH_EASTERN_GREAT_SWAMP,
    ]:
        swamp_dw = context.spritesets[area.dark_world.spriteset_id]
        swamp1 = context.spritesets[area.light_world_v1.spriteset_id]
        swamp2 = context.spritesets[area.light_world_v2.spriteset_id]
        lw_sheet3 = swamp2.sheet3
        swamp1.sheet3 = swamp_dw.sheet3
        swamp2.sheet3 = swamp_dw.sheet3
        swamp_dw = lw_sheet3


def invert_overworld_versions(context: Context, area: OverworldArea) -> None:
    if not area.dark_world or not area.light_world_v2:
        # Skip inversion for areas that don't have both a light and dark world.
        # It is assumed that light world v2 exists if any light world exists
        return

    if area.id in [
        OverworldAreaId.x16_WITCHS_HUT,
        OverworldAreaId.x28_KAKARIKO_VILLAGE_MAZE_RACE,
        OverworldAreaId.x18_KAKARIKO_VILLAGE,
        OverworldAreaId.x2A_HAUNTED_GROVE,
        OverworldAreaId.x40_MASTER_SWORD_UNDER_BRIDGE,
    ]:
        # Skip rooms we can't handle yet.
        return

    light_sprite_palette_id = area.light_world_v2.sprite_palette_id
    light_spriteset_id = area.light_world_v2.spriteset_id
    dark_sprite_palette_id = area.dark_world.sprite_palette_id
    dark_spriteset_id = area.dark_world.spriteset_id

    area.light_world_v1.sprite_palette_id = dark_sprite_palette_id
    area.light_world_v1.spriteset_id = dark_spriteset_id
    area.light_world_v2.sprite_palette_id = dark_sprite_palette_id
    area.light_world_v2.spriteset_id = dark_spriteset_id
    area.dark_world.sprite_palette_id = light_sprite_palette_id
    area.dark_world.spriteset_id = light_spriteset_id

    _apply_area_specific_tweaks(context, area)


def invert_world(context: Context) -> None:
    for area in context.overworld_areas.values():
        invert_overworld_versions(context, area)
