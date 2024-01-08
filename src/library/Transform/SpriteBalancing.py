from typing import List
from .Context import Context
from ..Model import SpriteId


def _compute_difficulty(context: Context, sprite_id) -> int:
    return 0


def get_weights_random(_: Context, sprite_ids: List[SpriteId]) -> List[int]:
    return [1 for _ in sprite_ids]


def get_weights_balanced(context: Context, sprite_ids: List[SpriteId]) -> List[int]:
    # Find difficulty by multiplying the green mail damage by the creature's HP.
    # This seems to more or less line up with difficulty of killing the creature
    # Min HP for this purpose is 4. Damage has a +1 modifier to prevent 0.
    sprite_ids.sort(
        key=lambda x: (
            context.damage_table.link_damage_rows[
                context.sprites[x].subclass
            ].green_mail
            + 1
        )
        * max(context.sprites[x].hp, 4),
        reverse=True,
    )
    # create weight which is its fractional place in the list. This means enemies with
    # high difficulty relative to the group will be placed slightly less often.
    # Add 0.25 so the weight is never 0 (not placed). This also serves to balance the weightings somewhat.
    return [
        max(0, 0.25 + (index / len(sprite_ids))) for index, _ in enumerate(sprite_ids)
    ]


def get_weights_casual(context: Context, sprite_ids: List[SpriteId]) -> List[int]:
    # Find difficulty by multiplying the green mail damage by the creature's HP.
    # This seems to more or less line up with difficulty of killing the creature
    # Min HP for this purpose is 4. Damage has a +1 modifier to prevent 0.
    sprite_ids.sort(
        key=lambda x: (
            context.damage_table.link_damage_rows[
                context.sprites[x].subclass
            ].green_mail
            + 1
        )
        * max(context.sprites[x].hp, 4),
        reverse=False,
    )
    # Zero out the top 70% of enemies.
    count = len(sprite_ids)
    return [
        max(0.00001, (5 if (index + 1 / count) > .666 else 0))
        for index, _ in enumerate(sprite_ids)
    ]



def get_weights_hero(context: Context, sprite_ids: List[SpriteId]) -> List[int]:
    # Find difficulty by multiplying the green mail damage by the creature's HP.
    # This seems to more or less line up with difficulty of killing the creature
    # Min HP for this purpose is 4. Damage has a +1 modifier to prevent 0.
    sprite_ids.sort(
        key=lambda x: (
            context.damage_table.link_damage_rows[
                context.sprites[x].subclass
            ].green_mail
            + 1
        )
        * max(context.sprites[x].hp, 4),
        reverse=True,
    )
    # Zero out the top 70% of enemies.
    count = len(sprite_ids)
    return [
        max(0.00001, (5 if (index + 1 / count) > .666 else 0))
        for index, _ in enumerate(sprite_ids)
    ]
