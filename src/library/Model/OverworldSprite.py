from attr import dataclass

from library.Model.SpriteId import SpriteId


@dataclass
class OverworldSprite:
    _address: int
    """The ROM address of the Sprite in an Overworld Area. DO NOT MODIFY."""
    sprite_id: SpriteId
    """The Sprite placed in the Overworld Area."""
    x: int
    """The x coordinate of the Sprite in the Overworld Area."""
    y: int
    """The y coordinate of the Sprite in the Overworld Area."""
