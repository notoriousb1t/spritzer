from attr import dataclass
from enum import StrEnum
from random import Random
from typing import Dict, List, Set


from ..Model import (
    DamageTable,
    UnderworldRoom,
    UnderworldRoomId,
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


class Balancing(StrEnum):
    RANDOM = "random"
    CASUAL = "casual"
    BALANCED = "balanced"
    HERO = "hero"


@dataclass
class Context:
    random: Random
    loaded = False
    damage_table: DamageTable = None
    underworld_balancing = Balancing.BALANCED
    underworld_rooms: Dict[UnderworldRoomId, UnderworldRoom] = {}
    underworld_choices: Dict[SpritesetId, Set[SpriteId]] = {}
    overworld_areas: Dict[OverworldAreaId, OverworldArea] = {}
    overworld_balancing = Balancing.BALANCED
    overworld_choices: Dict[SpritesetId, Set[SpriteId]] = {}
    spritesets: Dict[SpritesetId, Spriteset] = {}
    sprites: Dict[SpriteId, Sprite] = {}
    sprite_subclasses: Dict[SpriteSubclassId, SpriteSubclass] = {}
    spritesheet_sprites: Dict[SpriteSheetId, List[SpriteId]] = {}
    unused_spritesets: List[SpritesetId] = []
