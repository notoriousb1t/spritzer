from typing import List

from library.model.spriteset_id import SpritesetId
from library.model.model import Model


def _get_area_spritesets() -> List[SpritesetId]:
    return [spriteset_id for spriteset_id in list(SpritesetId) if spriteset_id <= 0x40]


def create_free_overworld_spriteset_list(context: Model) -> List[SpritesetId]:
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


def create_free_underworld_spriteset_list(context: Model) -> List[SpritesetId]:
    """Returns a list of spritesheets that are considered empty and can be used for swapping."""
    return [
        SpritesetId.x2C_UNDERWORLD_FREESPACE,
        SpritesetId.x2D_UNDERWORLD_FREESPACE,
        SpritesetId.x2E_UNDERWORLD_FREESPACE,
        SpritesetId.x2F_UNDERWORLD_FREESPACE,
        SpritesetId.x30_UNDERWORLD_FREESPACE,
        SpritesetId.x31_UNDERWORLD_FREESPACE,
        SpritesetId.x32_UNDERWORLD_FREESPACE,
        SpritesetId.x33_UNDERWORLD_FREESPACE,
        SpritesetId.x34_UNDERWORLD_FREESPACE,
        SpritesetId.x35_UNDERWORLD_FREESPACE,
        SpritesetId.x36_UNDERWORLD_FREESPACE,
        SpritesetId.x37_UNDERWORLD_FREESPACE,
        SpritesetId.x38_UNDERWORLD_FREESPACE,
        SpritesetId.x39_UNDERWORLD_FREESPACE,
        SpritesetId.x3A_UNDERWORLD_FREESPACE,
        SpritesetId.x3B_UNDERWORLD_FREESPACE,
        SpritesetId.x3C_UNDERWORLD_FREESPACE,
        SpritesetId.x3D_UNDERWORLD_FREESPACE,
    ]
