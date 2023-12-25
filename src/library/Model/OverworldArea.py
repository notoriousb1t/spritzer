from attr import dataclass
from typing import List

from .OverworldAreaId import OverworldAreaId
from .OverworldSprite import OverworldSprite
from .SpriteBlocksetId import SpriteBlocksetId


@dataclass
class OverworldArea:
    id: OverworldAreaId
    """The Area this block describes. DO NOT MODIFY."""
    blockset_id: SpriteBlocksetId
    """The graphics block associated. This constrains which Entities can appear in this Area."""
    overworld_sprites: List[OverworldSprite]
    """The Entities in this Overworld Area. The size of this list MUST remain fixed."""
    sprite_blockset_address: int
