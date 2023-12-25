from attr import dataclass
from .SpriteId import SpriteId


@dataclass
class DungeonSprite:
    _address: int
    """The ROM address of the Sprite in an Dungeon Room. DO NOT MODIFY."""
    sprite_id: SpriteId
    """The type of Sprite placed in the Dungeon Room.."""
    y: int
    """The y coordinate of the Sprite in the Dungeon Room. and some settings."""
    x: int
    """The x coordinate of the Sprite in the Dungeon Room. and some settings."""
    is_overlord: bool
    """True if this Sprite has the overlord bits set."""
    is_subtype: bool
    """True if this Sprite has the subtype bits set."""
    item: SpriteId
    """Contains another Sprite. This should be a key or big key."""
    distance_from_midpoint: int = 0
    """May be set when performing shuffling/randomization to evaluate distance from the center of a group of enemies."""

    def has_key(self) -> bool:
        return self.item == SpriteId.xE4_KEY or self.item == SpriteId.xE5_BIG_KEY