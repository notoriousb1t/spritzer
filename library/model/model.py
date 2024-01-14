from library.model.balancing import Balancing
from library.model.damage_subclass import DamageSubclass
from library.model.damage_subclass_id import DamageSubclassId
from library.model.damage_table import DamageTable
from library.model.overworld_area import OverworldArea
from library.model.overworld_area_id import OverworldAreaId
from library.model.sprite import Sprite
from library.model.sprite_id import SpriteId
from library.model.sprite_sheet_id import SpriteSheetId
from library.model.spriteset import Spriteset
from library.model.spriteset_id import SpritesetId
from library.model.underworld_room import UnderworldRoom
from library.model.underworld_room_id import UnderworldRoomId


from attr import dataclass


from typing import Dict, List, Set


@dataclass
class Model:
    seed: str
    """Used for pseudo-randomization"""
    damage_table: DamageTable
    """Contains the main damage table for computing damaged against Link."""
    underworld_balancing: Balancing
    """The general difficulty of randomization of sprites in the underworld"""
    underworld_rooms: Dict[UnderworldRoomId, UnderworldRoom]
    """The current set of rooms in caves, dungeons, etc. in the underworld"""
    overworld_areas: Dict[OverworldAreaId, OverworldArea]
    """The current set of areas in the overworld map"""
    overworld_balancing: Balancing
    """The general difficulty of randomization of sprites in the overworld"""
    spritesets: Dict[SpritesetId, Spriteset]
    """The Spriteset blocks used to rendered sprites. There are 4 associated with each ID."""
    sprites: Dict[SpriteId, Sprite]
    """The Settings of each Sprite. Please note that the game logic may have hard coded values as well."""
    sprite_subclasses: Dict[DamageSubclassId, DamageSubclass]
    """The amount of damage each weapon type creates at base value."""
    spritesheet_sprites: Dict[SpriteSheetId, List[SpriteId]]
    """A mapping between sprite sheets used in spritesets and which sprites they represent"""
    unused_spritesets: List[SpritesetId] = []
    """A list of remaining spritesets that can be mapped to overworld sprite sheets"""
    overworld_choices: Dict[SpritesetId, Set[SpriteId]] = {}
    """The list of sprites usable in each spriteset in the overworld"""
    underworld_choices: Dict[SpritesetId, Set[SpriteId]] = {}
    """The list of sprites usable in each spriteset in the underworld"""

