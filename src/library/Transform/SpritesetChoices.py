from typing import List
from ..Model import SpritesetId
from .Context import Context


def _get_area_spritesets() -> List[SpritesetId]:
    return [spriteset_id for spriteset_id in list(SpritesetId) if spriteset_id <= 0x40]


def create_free_spriteset_list(context: Context) -> List[SpritesetId]:
    """Returns a list of spritesheets that are considered empty and can be used for swapping."""
    return [
        id
        for id in _get_area_spritesets()
        if not id in context.spritesets.keys()
        # Last vanilla address
        or id > 0x25
        # Check if the spritesheets are empty.
        or (
            context.spritesets[id].sheet0 == 0
            and context.spritesets[id].sheet1 == 0
            and context.spritesets[id].sheet2 == 0
            and context.spritesets[id].sheet3 == 0
        )
    ]


def create_free_dungeon_spriteset_list(context: Context) -> List[SpritesetId]:
    """Returns a list of spritesheets that are considered empty and can be used for swapping."""
    return [
        SpritesetId.x2C_DUNGEON_FREESPACE,
        SpritesetId.x2D_DUNGEON_FREESPACE,
        SpritesetId.x2E_DUNGEON_FREESPACE,
        SpritesetId.x2F_DUNGEON_FREESPACE,
        SpritesetId.x30_DUNGEON_FREESPACE,
        SpritesetId.x31_DUNGEON_FREESPACE,
        SpritesetId.x32_DUNGEON_FREESPACE,
        SpritesetId.x33_DUNGEON_FREESPACE,
        SpritesetId.x34_DUNGEON_FREESPACE,
        SpritesetId.x35_DUNGEON_FREESPACE,
        SpritesetId.x36_DUNGEON_FREESPACE,
        SpritesetId.x37_DUNGEON_FREESPACE,
        SpritesetId.x38_DUNGEON_FREESPACE,
        SpritesetId.x39_DUNGEON_FREESPACE,
        SpritesetId.x3A_DUNGEON_FREESPACE,
        SpritesetId.x3B_DUNGEON_FREESPACE,
        SpritesetId.x3C_DUNGEON_FREESPACE,
        SpritesetId.x3D_DUNGEON_FREESPACE,
    ]
