from typing import Dict, Set
from .Context import Context
from ..Model import SpriteId, SpritesetId


def compute_sprite_choices(context: Context) -> Dict[SpritesetId, Set[SpriteId]]:
    # Create a dictionary of Entities which occur in that graphics blocks in these Overworld Areas.
    choices: Dict[SpritesetId, Set[SpriteId]] = {}

    for spriteset_id, spriteset in context.spritesets.items():
        if not spriteset_id in choices:
            choices[spriteset_id] = set()
        all_sprites = [
            sprite_id
            for spritesheet in [
                spriteset.sheet0,
                spriteset.sheet1,
                spriteset.sheet2,
                spriteset.sheet3,
            ]
            for sprite_id in (
                context.spritesheet_sprites[spritesheet]
                if spritesheet in context.spritesheet_sprites
                else []
            )
        ]
        if SpriteId.xE_SNAPDRAGON in all_sprites:
            # Remove a snapdragon. If there is one left, then it is actually
            # a choice. Snapdragon has two required spritesheets.
            all_sprites.remove(SpriteId.xE_SNAPDRAGON)

        for sprite_id in all_sprites:
            choices[spriteset_id].add(sprite_id)

    # for room in context.dungeon_rooms.values():
    #     if not room.spriteset_id in choices:
    #         choices[room.spriteset_id] = set()
    #     for sprite in room.sprites:
    #         choices[room.spriteset_id].add(sprite.sprite_id)

    return choices
