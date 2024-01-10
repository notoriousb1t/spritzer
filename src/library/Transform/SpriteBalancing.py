from typing import List

from library.Model.SpriteId import SpriteId
from library.Model.SpriteType import SpriteType
from library.Transform.Context import Context, Balancing


_casual_blocklist = [
    SpriteId.x43_RED_SPEAR_SOLDIER,
    SpriteId.x48_RED_JAVELIN_SOLDIER,
    SpriteId.x49_RED_JAVELIN_SOLDIER_2,
    SpriteId.x4A_RED_BOMB_SOLDIERS,
    SpriteId.xD0_LYNEL,
]

_hero_blocklist = [
    SpriteId.x42_GREEN_SWORD_SOLDIER,
    SpriteId.x45_GREEN_SPEAR_SOLDIER,
    SpriteId.x47_GREEN_ARCHER,
    SpriteId.x4B_GREEN_SOLDIER_RECRUITS,
    SpriteId.x27_DEADROCK,
    SpriteId.xC4_THIEF,
]

# Used in balanced to reduce harder to kill enemies who rank lower
# in difficulty but are perceived to be more difficult.
_adjusted_enemies = [
    SpriteId.x43_RED_SPEAR_SOLDIER,
    SpriteId.x48_RED_JAVELIN_SOLDIER,
    SpriteId.x49_RED_JAVELIN_SOLDIER_2,
    SpriteId.xD0_LYNEL,
]


def _compute_difficulty(context: Context, sprite_id: SpriteId) -> int:
    sprite = context.sprites[sprite_id]
    subclass = sprite.subclass if sprite.subclass != None else 1
    hp = sprite.hp if sprite.hp != None and sprite.hp != 0xFF else 0x10

    if sprite_id in _adjusted_enemies:
        # Artificially make Lynels harder because they are.
        subclass += 2

    if sprite_id.requires_weapon:
        # Enemies that require specific weapons are harder
        # to kill, but usually have lower HP despite that
        hp *= 2

    return (hp / 4) * subclass


def _get_weights_random(_: Context, sprite_ids: List[SpriteId]) -> List[int]:
    return [1 for _ in sprite_ids]


def _get_weights_balanced(context: Context, sprite_ids: List[SpriteId]) -> List[int]:
    """This setting prevents a lot of very difficult enemies from spawning while not being super easy."""
    sprite_ids.sort(key=lambda x: _compute_difficulty(context, x))
    count = len(sprite_ids)
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
        for index, id in enumerate(sprite_ids)
    ]


def _get_weights_casual(context: Context, sprite_ids: List[SpriteId]) -> List[int]:
    sprite_ids.sort(key=lambda x: _compute_difficulty(context, x))
    # Zero out the top 50% of enemies. Also removed any enemy with red in its name (soldiers)
    count = len(sprite_ids)
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
        for index, id in enumerate(sprite_ids)
    ]


def _get_weights_hero(context: Context, sprite_ids: List[SpriteId]) -> List[int]:
    sprite_ids.sort(key=lambda x: _compute_difficulty(context, x))
    # Zero out the bottom 50% of enemies. Also removed any enemy with green in its name (soldiers)
    count = len(sprite_ids)
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
        for index, id in enumerate(sprite_ids)
    ]


def get_weights(
    balancing: Balancing,
    context: Context,
    sprite_ids: List[SpriteId],
) -> List[int]:
    if balancing == Balancing.RANDOM:
        return _get_weights_random(context, sprite_ids)
    if balancing == Balancing.BALANCED:
        return _get_weights_balanced(context, sprite_ids)
    if balancing == Balancing.CASUAL:
        return _get_weights_casual(context, sprite_ids)
    if balancing == Balancing.HERO:
        return _get_weights_hero(context, sprite_ids)
    raise "Invalid Balancing!"
