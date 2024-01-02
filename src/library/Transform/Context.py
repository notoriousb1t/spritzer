from attr import dataclass
from random import Random
from typing import Dict, List


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
    overworld_areas: Dict[OverworldAreaId, OverworldArea] = {}
    choices: Dict[SpritesetId, SpriteId] = {}
    spritesets: Dict[SpritesetId, Spriteset] = {}
    sprites: Dict[SpriteId, Sprite] = {}
    sprite_subclasses: Dict[SpriteSubclassId, SpriteSubclass] = {}
    spritesheet_sprites: Dict[SpriteSheetId, List[SpriteId]] = {}
    free_spritesheets: List[SpriteSheetId] = []
