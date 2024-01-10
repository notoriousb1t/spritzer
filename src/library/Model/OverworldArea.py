from attr import dataclass
from typing import List

from library.Model.OverworldAreaId import OverworldAreaId
from library.Model.OverworldId import OverworldId
from library.Model.OverworldSprite import OverworldSprite
from library.Model.SpritesetId import SpritesetId


@dataclass
class OverworldAreaVersion:
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
    lw_v1: OverworldAreaVersion = None
    """This is used after rescuing Zelda."""
    lw_v2: OverworldAreaVersion = None
    """This is used after defeating Agahnim."""
    dw: OverworldAreaVersion = None
    """This is used in the dark world."""

    @property
    def versions(self) -> List[OverworldAreaVersion]:
        configs: List[OverworldAreaVersion] = list()
        if self.lw_v1:
            configs.append(self.lw_v1)
        if self.lw_v2:
            configs.append(self.lw_v2)
        if self.dw:
            configs.append(self.dw)
        return configs

    @property
    def all_sprites(self) -> List[OverworldSprite]:
        return [sprite for versions in self.versions for sprite in versions.sprites]
