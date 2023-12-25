from attr import dataclass
from typing import List

from .DungeonRoomId import DungeonRoomId
from .DungeonRoomTilesetId import DungeonRoomTilesetId
from .DungeonSprite import DungeonSprite
from .DungeonTag import DungeonTag
from .PaletteId import PaletteId
from .SpriteBlocksetId import SpriteBlocksetId


@dataclass
class DungeonRoom:
    id: DungeonRoomId
    """The Dungeon Room this block of data describes. DO NOT MODIFY."""
    header_address: int
    """The ROM address of the Dungeon Room. DO NOT MODIFY."""
    lights_out_effect: bool
    """True if the lights are out for this Dungeon Room."""
    palette_id: PaletteId
    """The palette to load for the Dungeon Room."""
    tileset_id: DungeonRoomTilesetId
    """Unused for now, probably the graphics id associated with the tileset."""
    blockset_id: SpriteBlocksetId
    """The sprite graphics block associated. This constrains which Sprites can appear in this Dungeon Room."""
    effect: int
    """The visual effect of the Dungeon Room."""
    tag1: DungeonTag
    """The first tag. This provides data such as kill conditions."""
    tag2: DungeonTag
    """The second tag. This provides data such as kill conditions."""
    sprite_ptr: (int, int)
    """Used for dungeon room swaps. DO NOT MODIFY"""
    dungeon_sprites: List[DungeonSprite]
    """List of sprites in this Dungeon Room."""
