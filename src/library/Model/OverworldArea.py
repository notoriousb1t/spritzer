from attr import dataclass
from typing import List

from .OverworldId import OverworldId
from .OverworldAreaId import OverworldAreaId
from .OverworldSprite import OverworldSprite
from .SpritesetId import SpritesetId

@dataclass
class OverworldAreaRoom:
    overworld_id: OverworldId
    """The overworld state this room is for."""
    spriteset_id: SpritesetId
    """The graphics block associated. This constrains which Entities can appear in this Area."""
    sprite_palette_id: int
    """Sprite Palette ID."""
    sprites: List[OverworldSprite]
    """The Entities in this Overworld Area. The size of this list MUST remain fixed."""


@dataclass
class OverworldArea:
    id: OverworldAreaId
    """The Area this block describes. DO NOT MODIFY."""
    light_world_v1: OverworldAreaRoom = None
    """This is used after rescuing Zelda."""
    light_world_v2: OverworldAreaRoom = None
    """This is used after defeating Agahnim."""
    dark_world: OverworldAreaRoom = None
    """This is used in the dark world."""

    @property
    def versions(self) -> List[OverworldAreaRoom]:
        configs: List[OverworldAreaRoom] = list()
        if self.light_world_v1:
            configs.append(self.light_world_v1)
        if self.light_world_v2:
            configs.append(self.light_world_v2)
        if self.dark_world:
            configs.append(self.dark_world)
        return configs
