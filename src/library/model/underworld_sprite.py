from attr import dataclass
from typing import Union
from library.model.sprite_id import SpriteId


@dataclass
class UnderworldSprite:
    address: int
    """The ROM address of the Sprite in an Underworld Room. DO NOT MODIFY."""
    sprite_id: SpriteId
    """The type of Sprite placed in the Underworld Room.."""
    y: int
    """The y coordinate of the Sprite in the Underworld Room. and some settings."""
    x: int
    """The x coordinate of the Sprite in the Underworld Room. and some settings."""
    lower_layer: bool
    """True if this sprite is rendered on the bottom layer."""
    item: Union[SpriteId, None]
    """Contains another Sprite. This should be a key or big key."""
    aux0: int
    """Subtype information about the sprite. Varies per sprite"""
    aux1: int
    """A non-zero value indicates an overlord or key usually."""
    distance_from_midpoint: int = 0
    """May be set when performing shuffling/randomization to evaluate distance from the center of a group of enemies."""

    def has_key(self) -> bool:
        return self.item == SpriteId.xE4_KEY or self.item == SpriteId.xE5_BIG_KEY
