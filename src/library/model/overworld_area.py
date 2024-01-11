from attr import dataclass
from typing import List, Union
from library.model.overworld_area_id import OverworldAreaId
from library.model.overworld_id import OverworldId
from library.model.overworld_sprite import OverworldSprite
from library.model.spriteset_id import SpritesetId


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
    lw_v2: OverworldAreaVersion
    """This is used after defeating Agahnim."""
    lw_v1: Union[OverworldAreaVersion, None] = None
    """This is used after rescuing Zelda."""
    dw: Union[OverworldAreaVersion, None] = None
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
