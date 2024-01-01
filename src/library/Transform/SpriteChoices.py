from typing import Dict, Set
from .Context import Context
from ..Model import SpriteId, SpritesetId


def compute_sprite_choices(context: Context) -> Dict[SpritesetId, Set[SpriteId]]:
    # Create a dictionary of Entities which occur in that graphics blocks in these Overworld Areas.
    choices: Dict[SpritesetId, Set[SpriteId]] = {}

    for area in context.overworld_areas.values():
        for version in area.versions:
            if not version.spriteset_id in choices:
                choices[version.spriteset_id] = set()
            for sprite in version.sprites:
                choices[version.spriteset_id].add(sprite.sprite_id)

    for room in context.dungeon_rooms.values():
        if not room.spriteset_id in choices:
            choices[room.spriteset_id] = set()
        for sprite in room.sprites:
            choices[room.spriteset_id].add(sprite.sprite_id)

    return choices