from random import Random
from typing import List, Set


from . import Context, Placement, is_compatible
from ..Model import (
    SpriteId,
    OverworldSprite,
)


def _balanced_weights(context: Context, sprite_ids: List[SpriteId]) -> List[int]:
    """This is an attempt at balancing a list of enemies to reduce likelihood of too many hard enemies"""
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
    # create weight which is 1 + its fractional value in the list. This means enemies with
    # high hit points relative to the group will be placed less often.
    # Add 0.25 so the weight is never 0. This also serves to balance the weightings somewhat.
    return [0.25 + (index / len(sprite_ids)) for index, _ in enumerate(sprite_ids)]


def _reroll_overworld_sprites(
    context: Context,
    random: Random,
    overworld_sprites: List[OverworldSprite],
    choices: Set[SpriteId],
) -> None:
    if len(choices) < 1:
        return

    for overworld_sprite in overworld_sprites:
        # Make 3 attempts to replace the original with something else. This is biased,
        # but if too much looks vanilla, it doesn't look particularly random.
        for _ in range(0, 2):
            # Find all normal replacements
            possible_matches = [
                target_sprite_id
                for target_sprite_id in list(choices)
                if is_compatible(
                    overworld_sprite.sprite_id, target_sprite_id, Placement.AREA
                )
            ]

            if len(possible_matches) == 0:
                sprite_id = SpriteId.xE3_FAIRY
                break

            # Try to find a suitable match, if not just leave the Sprite as is.
            weights = _balanced_weights(context, possible_matches)
            sprite_id = random.choices(possible_matches, weights=weights)[0]
            if sprite_id != overworld_sprite.sprite_id:
                overworld_sprite.sprite_id = sprite_id


def reroll_overworld_enemies(context: Context) -> None:
    random = context.random

    # Randomize using Entities that occur anywhere in that Overworld Area.
    for overworld_area in context.overworld_areas.values():
        # Reroll all Overworld Areas using the choices collected from related Overworld Areas.
        for version in overworld_area.versions:
            # Get the possibly Overworld Sprites from the current graphics block
            choices = context.overworld_choices[version.spriteset_id]
            _reroll_overworld_sprites(
                context,
                random,
                version.sprites,
                choices,
            )
