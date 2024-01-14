from math import floor
from typing import List, Union
from library.model.balancing import Balancing
from library.model.damage_subclass_id import DamageSubclassId
from library.model.sprite import Sprite
from library.model.sprite_id import SpriteId
from library.model.sprite_type import SpriteType
from library.model.model import Model


_casual_blocklist: list[SpriteId] = [
    SpriteId.x43_RED_SPEAR_GUARD,
    SpriteId.x48_RED_JAVELIN_GUARD,
    SpriteId.x49_RED_GUARD_BUSH,
    SpriteId.x4A_RED_BOMB_GUARD,
    SpriteId.xD0_LYNEL,
]

_hero_blocklist: list[SpriteId] = [
    SpriteId.x42_GREEN_GUARD,
    SpriteId.x45_GREEN_ASSAULT_GUARD,
    SpriteId.x47_GREEN_GUARD_BUSH,
    SpriteId.x4B_GREEN_KNIFE_GUARD,
    SpriteId.x27_DEADROCK,
    SpriteId.xC4_THIEF,
]

# Used in balanced to reduce harder to kill enemies who rank lower
# in difficulty but are perceived to be more difficult.
_adjusted_enemies: list[SpriteId] = [
    SpriteId.x43_RED_SPEAR_GUARD,
    SpriteId.x48_RED_JAVELIN_GUARD,
    SpriteId.x49_RED_GUARD_BUSH,
    SpriteId.xD0_LYNEL,
]


def _compute_difficulty(context: Model, sprite_id: SpriteId) -> float:
    sprite: Union[Sprite, None] = context.sprites[sprite_id] if sprite_id in context.sprites else None
    if sprite == None:
        return 0

    subclass: DamageSubclassId = sprite.subclass
    hp: int = sprite.hp

    if sprite_id in _adjusted_enemies:
        # Artificially make Lynels harder because they are.
        subclass = DamageSubclassId(value=subclass + 2)

    if sprite_id.requires_weapon:
        # Enemies that require specific weapons are harder
        # to kill, but usually have lower HP despite that
        hp *= 2

    return floor(hp / 4) * subclass


def _get_weights_random(_: Model, sprite_ids: List[SpriteId]) -> List[float]:
    return [1 for _ in sprite_ids]


def _get_weights_balanced(context: Model, sprite_ids: List[SpriteId]) -> List[float]:
    """This setting prevents a lot of very difficult enemies from spawning while not being super easy."""
    sprite_ids.sort(key=lambda x: _compute_difficulty(context=context, sprite_id=x))
    count: int = len(sprite_ids)
    return [
        max(
            0.000001,
            1
            if id.role == SpriteType.HAZARD
            else 2
            if (index / count) < 0.3
            else 0.1
            if (index / count) > 0.5 or id in _adjusted_enemies
            else 5,
        )
        for index, id in enumerate(iterable=sprite_ids)
    ]


def _get_weights_casual(context: Model, sprite_ids: List[SpriteId]) -> List[float]:
    sprite_ids.sort(key=lambda x: _compute_difficulty(context, x))
    # Zero out the top 50% of enemies. Also removed any enemy with red in its name (soldiers)
    count: int = len(sprite_ids)
    return [
        max(
            0.000001,
            (
                1
                if id.role == SpriteType.HAZARD
                else 0
                if (index / count) > 0.3
                else 0.000001
                if id in _casual_blocklist
                else 5
            ),
        )
        for index, id in enumerate(iterable=sprite_ids)
    ]


def _get_weights_hero(context: Model, sprite_ids: List[SpriteId]) -> List[float]:
    sprite_ids.sort(key=lambda x: _compute_difficulty(context=context, sprite_id=x))
    # Zero out the bottom 50% of enemies. Also removed any enemy with green in its name (soldiers)
    count: int = len(sprite_ids)
    return [
        max(
            0.00001,
            (
                1
                if id.role == SpriteType.HAZARD
                else 0
                if (index / count) <= 0.7
                else 5
                if not id in _hero_blocklist
                else 0.00001
            ),
        )
        for index, id in enumerate(iterable=sprite_ids)
    ]


def get_weights(
    balancing: Balancing,
    context: Model,
    sprite_ids: List[SpriteId],
) -> List[float]:
    if balancing == Balancing.RANDOM:
        return _get_weights_random(context, sprite_ids=sprite_ids)
    if balancing == Balancing.BALANCED:
        return _get_weights_balanced(context=context, sprite_ids=sprite_ids)
    if balancing == Balancing.CASUAL:
        return _get_weights_casual(context=context, sprite_ids=sprite_ids)
    if balancing == Balancing.HERO:
        return _get_weights_hero(context=context, sprite_ids=sprite_ids)
    raise "Invalid Balancing!" # type: ignore
