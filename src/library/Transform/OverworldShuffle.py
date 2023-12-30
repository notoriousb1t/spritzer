from random import Random
from typing import List, Dict, Set

from ..Model.SpritesetId import SpritesetId

from ..Model.OverworldSprite import OverworldSprite

from . import Context, Placement, is_compatible
from ..Model import OverworldArea, SpriteId


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
    gfx_groups: Dict[SpritesetId, List[OverworldArea]] = {
        it: list() for it in list(SpritesetId)
    }
    for overworld_area in overworld_area_dict.values():
        gfx_groups[overworld_area.spriteset_id].append(overworld_area)

    # Create a dictionary of Entities which occur in that graphics blocks in these Overworld Areas.
    gfx_choices: Dict[SpritesetId, List[SpriteId]] = {
        it: list() for it in list(SpritesetId)
    }
    for gfx, overworld_areas in gfx_groups.items():
        # Capture possible Overworld Sprites in this graphics block.
        choice_set: Set[SpriteId] = set()
        for overworld_area in overworld_areas:
            for overworld_sprite in overworld_area.overworld_sprites:
                choice_set.add(overworld_sprite.id)
        gfx_choices[gfx] = choice_set

    # Randomize using Entities that occur anywhere in that Overworld Area.
    for overworld_area_dict in gfx_groups.values():
        # Reroll all Overworld Areas using the choices collected from related Overworld Areas.
        for overworld_area in overworld_area_dict:
            # Get the possibly Overworld Sprites from the current graphics block
            choices = gfx_choices[overworld_area.spriteset_id]
            _reroll_overworld_sprites(random, overworld_area.overworld_sprites, choices)
