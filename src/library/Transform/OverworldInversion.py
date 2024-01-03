from typing import List
from ..Model import (
    OverworldArea,
    OverworldAreaVersion,
    OverworldAreaId,
    SpriteType,
    Spriteset,
    SpriteSheetId,
)
from . import Context


def _apply_area_specific_tweaks(context: Context, area: OverworldArea) -> None:
    """Specific patches to preserve NPCs and objects, but non-restrictive overall."""
    if area.id == OverworldAreaId.x30_DESERT_OF_MYSTERY:
        desert = context.spritesets[area.light_world_v2.spriteset_id]
        misery_mire = context.spritesets[area.dark_world.spriteset_id]
        temp = desert.sheet2
        desert.sheet2 = misery_mire.sheet2
        misery_mire.sheet2 = temp
    elif area.id == OverworldAreaId.x0_LOST_WOODS:
        lost_woods1 = context.spritesets[area.light_world_v1.spriteset_id]
        lost_woods2 = context.spritesets[area.light_world_v2.spriteset_id]
        lost_woods1.sheet2 = SpriteSheetId.x11_MISC_FAKE_SWORD
        lost_woods2.sheet2 = SpriteSheetId.x11_MISC_FAKE_SWORD
    elif area.id == OverworldAreaId.x3_WEST_DEATH_MOUNTAIN:
        death_mountain1 = context.spritesets[area.light_world_v1.spriteset_id]
        death_mountain2 = context.spritesets[area.light_world_v2.spriteset_id]
        death_mountain1.sheet2 = SpriteSheetId.x12_DESERT_1
        death_mountain1.sheet3 = SpriteSheetId.x10_MISC_ROCKS
        death_mountain2.sheet2 = SpriteSheetId.x12_DESERT_1
        death_mountain2.sheet3 = SpriteSheetId.x10_MISC_ROCKS
    elif area.id == OverworldAreaId.x1E_EASTERN_PALACE:
        palace_of_darkness = context.spritesets[area.dark_world.spriteset_id]
        palace_of_darkness.sheet3 = SpriteSheetId.x59_FOLLOWERS
    elif area.id == OverworldAreaId.x3A_PATH_BETWEEN_DESERT_OF_MYSTERY_AND_GREAT_SWAMP:
        path1 = context.spritesets[area.light_world_v1.spriteset_id]
        path2 = context.spritesets[area.light_world_v2.spriteset_id]
        path1.sheet3 = SpriteSheetId.x59_FOLLOWERS
        path2.sheet3 = SpriteSheetId.x59_FOLLOWERS
    elif area.id == OverworldAreaId.x16_WITCHS_HUT:
        hut1 = context.spritesets[area.light_world_v1.spriteset_id]
        hut2 = context.spritesets[area.light_world_v2.spriteset_id]
        hut1.sheet2 = SpriteSheetId.x4C_SAHASRAHLA_WITCH
        hut2.sheet2 = SpriteSheetId.x4C_SAHASRAHLA_WITCH
    elif area.id in [OverworldAreaId.x3B_SOUTH_WESTERN_GREAT_SWAMP, OverworldAreaId.x34_NORTH_EASTERN_GREAT_SWAMP]:
        swamp1 = context.spritesets[area.light_world_v1.spriteset_id]
        swamp2 = context.spritesets[area.light_world_v2.spriteset_id]
        swamp1.sheet3 = SpriteSheetId.x11_MISC_FAKE_SWORD
        swamp2.sheet3 = SpriteSheetId.x11_MISC_FAKE_SWORD
    

def invert_overworld_versions(context: Context, area: OverworldArea) -> None:
    if not area.dark_world or not area.light_world_v2:
        # Skip inversion for areas that don't have both a light and dark world.
        # It is assumed that light world v2 exists if any light world exists
        return
    
    if area.id == OverworldAreaId.x28_KAKARIKO_VILLAGE_MAZE_RACE:
        return
    if area.id == OverworldAreaId.x18_KAKARIKO_VILLAGE:
        return
    if area.id == OverworldAreaId.x2A_HAUNTED_GROVE:
        return
    if area.id == OverworldAreaId.x40_MASTER_SWORD_UNDER_BRIDGE:
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
