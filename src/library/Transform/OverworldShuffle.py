from random import Random
from typing import List, Set


from . import Context, Placement, is_compatible
from ..Model import (
    SpriteId,
    OverworldSprite,
)


def _reroll_overworld_sprites(
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

            # Try to find a suitable match, if not just leave the Sprite as is.
            sprite_id = (
                random.choice(possible_matches) if len(possible_matches) > 0 else None
            )
            if sprite_id == None:
                sprite_id = SpriteId.xE3_FAIRY
                break
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
            _reroll_overworld_sprites(random, version.sprites, choices)
