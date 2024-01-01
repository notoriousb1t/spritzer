from random import Random
from typing import List, Dict, Set

from . import Context, Placement, is_compatible
from ..Model import (
    SpriteId,
    OverworldSprite,
    SpritesetId,
    OverworldAreaRoom,
)


def _reroll_overworld_sprites(
    random: Random,
    overworld_sprites: List[OverworldSprite],
    choices: List[SpriteId],
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
                for target_sprite_id in choices
                if is_compatible(overworld_sprite.id, target_sprite_id, Placement.AREA)
            ]

            # Try to find a suitable match, if not just leave the Sprite as is.
            sprite_id = (
                random.choice(possible_matches) if len(possible_matches) > 0 else None
            )
            if sprite_id == None:
                break
            if sprite_id != overworld_sprite.id:
                overworld_sprite.id = sprite_id


def reroll_overworld(context: Context) -> None:
    random = context.random
    overworld_area_dict = context.overworld_areas

    # Group Overworld Areas by graphics block.
    gfx_groups: Dict[SpritesetId, List[OverworldAreaRoom]] = {
        it: list() for it in list(SpritesetId)
    }
    for overworld_area in overworld_area_dict.values():
        for configuration in overworld_area.versions:
            gfx_groups[configuration.spriteset_id].append(configuration)

    # Create a dictionary of Entities which occur in that graphics blocks in these Overworld Areas.
    gfx_choices: Dict[SpritesetId, List[SpriteId]] = {
        it: list() for it in list(SpritesetId)
    }
    for gfx, overworld_areas in gfx_groups.items():
        # Capture possible Overworld Sprites in this graphics block.
        choice_set: Set[SpriteId] = set()
        for configuration in overworld_areas:
            for overworld_sprite in configuration.sprites:
                choice_set.add(overworld_sprite.id)
        gfx_choices[gfx] = choice_set

    # Randomize using Entities that occur anywhere in that Overworld Area.
    for overworld_area_dict in gfx_groups.values():
        # Reroll all Overworld Areas using the choices collected from related Overworld Areas.
        for configuration in overworld_area_dict:
            # Get the possibly Overworld Sprites from the current graphics block
            choices = gfx_choices[configuration.spriteset_id]
            _reroll_overworld_sprites(random, configuration.sprites, choices)
