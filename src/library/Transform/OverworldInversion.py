from library.Model import OverworldArea, OverworldAreaVersion, SpriteId, SpriteType
from library.Transform import Context


def _can_invert_version(version: OverworldAreaVersion) -> bool:
    return all(
        not (
            sprite.sprite_id.meta().role
            in [SpriteType.NPC, SpriteType.OBJECT, SpriteType.OVERLORD]
        )
        for sprite in version.sprites
    )


def invert_overworld_versions(area: OverworldArea) -> None:
    if not area.dark_world or not area.light_world_v2:
        # Skip inversion for areas that don't have both a light and dark world.
        # It is assumed that light world v2 exists if any light world exists
        return

    light_sprite_palette_id = area.light_world_v2.sprite_palette_id
    light_spriteset_id = area.light_world_v2.spriteset_id
    dark_sprite_palette_id = area.dark_world.sprite_palette_id
    dark_spriteset_id = area.dark_world.spriteset_id

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
        invert_overworld_versions(area)