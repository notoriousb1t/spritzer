from typing import Dict, Set
from .Context import Context
from ..Model import SpriteId, SpritesetId


def _compute_basic_sprite_choices(context: Context) -> Dict[SpriteId, Set[SpriteId]]:
    """Compute sprite choices based on what is used from each Spriteset in vanilla."""
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


def _compute_full_sprite_choices(context: Context) -> Dict[SpritesetId, Set[SpriteId]]:
    """Compute sprite choices based on what is possible in a Spriteset."""
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
    return choices

def preprocess_simple_overworld_choices(context: Context) -> None:
    context.overworld_choices = _compute_basic_sprite_choices(context)
    
def preprocess_simple_dungeon_choices(context: Context) -> None:
    context.dungeon_choices = _compute_basic_sprite_choices(context)

def preprocess_full_overworld_choices(context: Context) -> None:
    context.overworld_choices = _compute_full_sprite_choices(context)

def preprocess_full_dungeon_choices(context: Context) -> None:
    context.dungeon_choices = _compute_full_sprite_choices(context)
