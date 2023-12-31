from attr import dataclass
from typing import List

from .BlocksetId import BlocksetId
from .UnderworldRoomFloorId import UnderworldRoomFloorId
from .UnderworldRoomId import UnderworldRoomId
from .UnderworldSprite import UnderworldSprite
from .UnderworldRoomTag import UnderworldRoomTag
from .PaletteId import PaletteId
from .SpritesetId import SpritesetId


@dataclass
class UnderworldRoom:
    id: UnderworldRoomId
    """The Underworld Room this block of data describes. DO NOT MODIFY."""
    header_address: int
    """The ROM address of the Underworld Room. DO NOT MODIFY."""
    bg2_property: int
    """True if the lights are out for this Underworld Room."""
    palette_id: PaletteId
    """The palette to load for the Underworld Room."""
    blockset_id: BlocksetId
    """Unused for now, probably the graphics id associated with the tileset."""
    spriteset_id: SpritesetId
    """The sprite graphics block associated. This constrains which Sprites can appear in this Underworld Room."""
    bgmove: int
    """The visual effect of the Underworld Room."""
    tag1: UnderworldRoomTag
    """The first tag. This provides data such as kill conditions."""
    tag2: UnderworldRoomTag
    """The second tag. This provides data such as kill conditions."""
    floor_upper: UnderworldRoomFloorId
    """The pattern/type of the top floor. (Water, Tiled, etc.)"""
    floor_lower: UnderworldRoomFloorId
    """The pattern/type of the bottom floor. (Water, Tiled, etc.)"""
    warp: UnderworldRoomId
    """The destination for pit warping"""
    stairs0: UnderworldRoomId
    """The destination for stairs 0"""
    stairs1: UnderworldRoomId
    """The destination for stairs 1"""
    stairs2: UnderworldRoomId
    """The destination for stairs 2"""
    stairs3: UnderworldRoomId
    """The destination for stairs 3"""
    sprite_ptr: (int, int)
    """Used for underworld room swaps. DO NOT MODIFY"""
    sprites: List[UnderworldSprite]
    """List of sprites in this Underworld Room."""
