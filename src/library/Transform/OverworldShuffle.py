from random import Random
from typing import List, Set

from . import Context, Placement, is_compatible
from ..Model import (
    SpriteId,
    OverworldSprite,
    OverworldArea,
    SpriteType,
    SpritesetId,
    OverworldAreaVersion,
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


def _can_invert_version(version: OverworldAreaVersion) -> bool:
    # Full inversion is allowed if there are no NPCs. Kiki is an exception
    # since the sprite is never seen as a non-follower.
    return all(
        (
            not sprite.sprite_id.meta().role
            in [SpriteType.NPC, SpriteType.OBJECT, SpriteType.OVERLORD]
        )
        for sprite in version.sprites
    )


def _prepare_spritesheets_to_preserve_requirements(
    context: Context, current_id: SpritesetId, next_id: SpriteId
) -> None:
    """This ensures that object, npcs, and overlords are preserved in inversions"""
    current_spriteset = context.spritesets[current_id]
    next_spriteset = context.spritesets[next_id]

    for sprite_id in context.spritesheet_sprites[current_spriteset.sheet0]:
        if sprite_id.meta().role in [
            SpriteType.NPC,
            SpriteType.OBJECT,
            SpriteType.OVERLORD,
            SpriteType.CREATURE,
        ]:
            temp0 = next_spriteset.sheet0
            next_spriteset.sheet0 = current_spriteset.sheet0
            current_spriteset.sheet0 = temp0
            break
    for sprite_id in context.spritesheet_sprites[current_spriteset.sheet1]:
        if sprite_id.meta().role in [
            SpriteType.NPC,
            SpriteType.OBJECT,
            SpriteType.OVERLORD,
            SpriteType.CREATURE,
        ]:
            temp1 = next_spriteset.sheet1
            next_spriteset.sheet1 = current_spriteset.sheet1
            current_spriteset.sheet1 = temp1
            break
    for sprite_id in context.spritesheet_sprites[current_spriteset.sheet2]:
        if sprite_id.meta().role in [
            SpriteType.NPC,
            SpriteType.OBJECT,
            SpriteType.OVERLORD,
            SpriteType.CREATURE,
        ]:
            temp2 = next_spriteset.sheet2
            next_spriteset.sheet2 = current_spriteset.sheet2
            current_spriteset.sheet2 = temp2
            break
    for sprite_id in context.spritesheet_sprites[current_spriteset.sheet3]:
        if sprite_id.meta().role in [
            SpriteType.NPC,
            SpriteType.OBJECT,
            SpriteType.OVERLORD,
            SpriteType.CREATURE,
        ]:
            temp3 = next_spriteset.sheet3
            next_spriteset.sheet3 = current_spriteset.sheet3
            current_spriteset.sheet3 = temp3
            break


def invert_overworld_versions(context, area: OverworldArea) -> None:
    if not area.dark_world or not area.light_world_v2:
        # Skip inversion for areas that don't have both a light and dark world.
        # It is assumed that light world v2 exists if any light world exists
        return

    light_sprite_palette_id = area.light_world_v2.sprite_palette_id
    light_spriteset_id = area.light_world_v2.spriteset_id
    dark_sprite_palette_id = area.dark_world.sprite_palette_id
    dark_spriteset_id = area.dark_world.spriteset_id

    # This ensures that required sprites end up rendering correctly.
    # _prepare_spritesheets_to_preserve_requirements(
    #     context, light_spriteset_id, dark_spriteset_id
    # )

    if area.light_world_v1 and _can_invert_version(area.light_world_v1):
        area.light_world_v1.sprite_palette_id = dark_sprite_palette_id
        area.light_world_v1.spriteset_id = dark_spriteset_id

    if _can_invert_version(area.light_world_v2):
        area.light_world_v2.sprite_palette_id = dark_sprite_palette_id
        area.light_world_v2.spriteset_id = dark_spriteset_id

    if _can_invert_version(area.dark_world):
        area.dark_world.sprite_palette_id = light_sprite_palette_id
        area.dark_world.spriteset_id = light_spriteset_id

    # Swap light world creatures with the first thing in the room.
    for version in area.versions:
        for sprite in version.sprites:
            if sprite.sprite_id.meta().role == SpriteType.CREATURE:
                replacement_id = next(
                    (
                        sprite.sprite_id
                        for sprite in version.sprites
                        if sprite.sprite_id.meta().role == SpriteType.ENEMY
                    ),
                    None,
                )
                if replacement_id == None:
                    # If we can't find a replacement, replace with a fairy.
                    # This may end up acting as a no-op.
                    sprite.sprite_id = SpriteId.xE3_FAIRY
                    continue
                sprite.sprite_id = replacement_id


def invert_world(context: Context) -> None:
    for area in context.overworld_areas.values():
        invert_overworld_versions(context, area)


def reroll_overworld_enemies(context: Context) -> None:
    random = context.random

    # Randomize using Entities that occur anywhere in that Overworld Area.
    for overworld_area in context.overworld_areas.values():
        # Reroll all Overworld Areas using the choices collected from related Overworld Areas.
        for version in overworld_area.versions:
            # Get the possibly Overworld Sprites from the current graphics block
            choices = context.choices[version.spriteset_id]
            _reroll_overworld_sprites(random, version.sprites, choices)
