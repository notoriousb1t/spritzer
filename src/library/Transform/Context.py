from attr import dataclass
from random import Random
from typing import Dict


from ..Model import (
    DungeonRoomId,
    DungeonRoom,
    OverworldAreaId,
    OverworldArea,
    SpriteBlocksetId,
    SpriteBlockset,
    SpriteId,
    Sprite,
)

@dataclass
class Context:
    random: Random
    loaded = False
    dungeon_rooms: Dict[DungeonRoomId, DungeonRoom] = {}
    overworld_areas: Dict[OverworldAreaId, OverworldArea] = {}
    sprite_blocksets: Dict[SpriteBlocksetId, SpriteBlockset] = {}
    sprites: Dict[SpriteId, Sprite] = {}

    def assert_unloaded(self) -> None:
        """This should be called when a Context is about to be loaded."""
        if self.loaded:
            raise "Cannot reuse the same instance of Context"

    def assert_loaded(self) -> None:
        """This should be called when a function accepts Context as a parameter."""
        if not self.loaded:
            raise "Please load Context before modifying"
