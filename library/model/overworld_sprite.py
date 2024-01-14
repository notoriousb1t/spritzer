from attr import dataclass

from library.model.sprite_id import SpriteId


@dataclass
class OverworldSprite:
    address: int
    """The ROM address of the Sprite in an Overworld Area. DO NOT MODIFY."""
    sprite_id: SpriteId
    """The Sprite placed in the Overworld Area."""
    x: int
    """The x coordinate of the Sprite in the Overworld Area."""
    y: int
    """The y coordinate of the Sprite in the Overworld Area."""
