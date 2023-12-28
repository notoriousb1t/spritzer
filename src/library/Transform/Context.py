from attr import dataclass
from random import Random
from typing import Dict


from ..Model import (
    DamageTable,
    DungeonRoom,
    DungeonRoomId,
    OverworldArea,
    OverworldAreaId,
    Spriteset,
    SpritesetId,
    SpriteId,
    Sprite,
    SpriteSubclass,
    SpriteSubclassId,
)


@dataclass
class Context:
    random: Random
    loaded = False
    damage_table: DamageTable = None
    dungeon_rooms: Dict[DungeonRoomId, DungeonRoom] = {}
    overworld_areas: Dict[OverworldAreaId, OverworldArea] = {}
    spritesets: Dict[SpritesetId, Spriteset] = {}
    sprites: Dict[SpriteId, Sprite] = {}
    sprite_subclasses: Dict[SpriteSubclassId, SpriteSubclass] = {}
