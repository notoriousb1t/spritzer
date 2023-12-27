from attr import dataclass
from typing import List

from .BlocksetId import BlocksetId
from .DungeonRoomFloorId import DungeonRoomFloorId
from .DungeonRoomId import DungeonRoomId
from .DungeonSprite import DungeonSprite
from .DungeonTag import DungeonTag
from .PaletteId import PaletteId
from .SpritesetId import SpritesetId


@dataclass
class DungeonRoom:
    id: DungeonRoomId
    """The Dungeon Room this block of data describes. DO NOT MODIFY."""
    header_address: int
    """The ROM address of the Dungeon Room. DO NOT MODIFY."""
    bg2_property: int
    """True if the lights are out for this Dungeon Room."""
    palette_id: PaletteId
    """The palette to load for the Dungeon Room."""
    blockset_id: BlocksetId
    """Unused for now, probably the graphics id associated with the tileset."""
    spriteset_id: SpritesetId
    """The sprite graphics block associated. This constrains which Sprites can appear in this Dungeon Room."""
    bgmove: int
    """The visual effect of the Dungeon Room."""
    tag1: DungeonTag
    """The first tag. This provides data such as kill conditions."""
    tag2: DungeonTag
    """The second tag. This provides data such as kill conditions."""
    floor_upper: DungeonRoomFloorId
    """The pattern/type of the top floor. (Water, Tiled, etc.)"""
    floor_lower: DungeonRoomFloorId
    """The pattern/type of the bottom floor. (Water, Tiled, etc.)"""
    warp: DungeonRoomId
    """The destination for pit warping"""
    stairs0: DungeonRoomId
    """The destination for stairs 0"""
    stairs1: DungeonRoomId
    """The destination for stairs 1"""
    stairs2: DungeonRoomId
    """The destination for stairs 2"""
    stairs3: DungeonRoomId
    """The destination for stairs 3"""
    sprite_ptr: (int, int)
    """Used for dungeon room swaps. DO NOT MODIFY"""
    dungeon_sprites: List[DungeonSprite]
    """List of sprites in this Dungeon Room."""
