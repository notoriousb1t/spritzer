from attr import dataclass
from random import Random
from typing import Dict


from ..Model import (
    DungeonRoom,
    DungeonRoomId,
    OverworldArea,
    OverworldAreaId,
    Spriteset,
    SpritesetId,
    SpriteId,
    Sprite,
)


@dataclass
class Context:
    random: Random
    loaded = False
    dungeon_rooms: Dict[DungeonRoomId, DungeonRoom] = {}
    overworld_areas: Dict[OverworldAreaId, OverworldArea] = {}
    spritesets: Dict[SpritesetId, Spriteset] = {}
    sprites: Dict[SpriteId, Sprite] = {}
