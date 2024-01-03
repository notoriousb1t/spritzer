from attr import dataclass
from random import Random
from typing import Dict, List, Set


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
    SpriteSheetId,
)


@dataclass
class Context:
    random: Random
    loaded = False
    damage_table: DamageTable = None
    dungeon_rooms: Dict[DungeonRoomId, DungeonRoom] = {}
    dungeon_choices: Dict[SpritesetId, Set[SpriteId]] = {}
    overworld_areas: Dict[OverworldAreaId, OverworldArea] = {}
    overworld_choices: Dict[SpritesetId, Set[SpriteId]] = {}
    spritesets: Dict[SpritesetId, Spriteset] = {}
    sprites: Dict[SpriteId, Sprite] = {}
    sprite_subclasses: Dict[SpriteSubclassId, SpriteSubclass] = {}
    spritesheet_sprites: Dict[SpriteSheetId, List[SpriteId]] = {}
