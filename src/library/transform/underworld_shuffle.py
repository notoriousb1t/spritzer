from math import floor
from random import Random
from typing import List, Dict, Set, Tuple
from library.model.sprite_id import SpriteId
from library.model.sprite_type import SpriteType
from library.model.underworld_room_id import UnderworldRoomId
from library.model.underworld_sprite import UnderworldSprite
from library.model.model import Model
from library.transform.compatibility import Placement, is_compatible
from library.transform.sprite_balancing import get_weights

_block_list: list[UnderworldRoomId] = [
    UnderworldRoomId.x24_TURTLE_ROCK_DOUBLE_HOKKU_BOKKU_BIG_CHEST_ROOM
]


def _find_distance(start: Tuple[int, int], end: Tuple[int, int]) -> int:
    return ((start[0] - end[0]) ** 2 + (start[1] - start[1]) ** 2) ** 0.5


def _detect_room_configuration(
    dungeon_sprites: List[UnderworldSprite],
) -> Tuple[int, int, bool]:
    start_x = 0
    start_y = 0
    end_x = 0
    end_y = 0
    max_distance = 0
    for i in range(len(dungeon_sprites)):
        for j in range(i + 1, len(dungeon_sprites)):
            distance: int = _find_distance(
                start=(dungeon_sprites[i].x, dungeon_sprites[i].y),
                end=(dungeon_sprites[j].x, dungeon_sprites[j].y),
            )
            if distance <= max_distance and max_distance != 0:
                continue
            max_distance: int = distance
            start: UnderworldSprite = dungeon_sprites[i if distance > 0 else j]
            start_x: int = start.x
            start_y: int = start.y
            end: UnderworldSprite = dungeon_sprites[i if distance <= 0 else j]
            end_x: int = end.x
            end_y: int = end.y

    is_horizontal: bool = abs(start_x - end_x) > abs(start_y - end_y)
    return (
        end_x - start_x,
        end_y - start_y,
        is_horizontal,
    )


def _sort_by_distance(dungeon_sprites: List[UnderworldSprite]) -> None:
    if len(dungeon_sprites) < 2:
        return

    # Find the center point between all sprites in this list.
    dungeon_room_config: Tuple[int, int, bool] = _detect_room_configuration(
        dungeon_sprites=dungeon_sprites
    )
    for dungeon_sprite in dungeon_sprites:
        if dungeon_room_config[2]:
            # Use the distance from midpoint on the y axis for horizontal configs.
            dungeon_sprite.distance_from_midpoint = floor(
                abs(dungeon_sprite.y - dungeon_room_config[1])
            )
        else:
            # Use the distance from midpoint on the x axis for vertical configs.
            dungeon_sprite.distance_from_midpoint = floor(
                abs(dungeon_sprite.x - dungeon_room_config[0])
            )

    # Sort the sprites so they are evaluated based on distance from the
    # center. This should place sprites nearest to the center in the
    # beginning of the list and further away ones at the end of the list
    # absolute value is uses so we can match up enemies that are symmetrical
    dungeon_sprites.sort(key=lambda it: it.distance_from_midpoint, reverse=False)


def _generate_sprite_selections(
    context: Model,
    random: Random,
    dungeon_room_sprites: List[UnderworldSprite],
    choices: Set[SpriteId],
    placement: Placement,
) -> Dict[int, SpriteId]:
    # Presort Dungeon Room Sprites by distance from the center.
    _sort_by_distance(dungeon_sprites=dungeon_room_sprites)

    distance_map: Dict[int, SpriteId] = {}
    for dungeon_room_sprite_group in [
        [it for it in dungeon_room_sprites if it.has_key()],
        [it for it in dungeon_room_sprites if not it.has_key()],
    ]:
        # Evaluate enemies with keys first so we can make sure that we always have keys accounted for
        # This ensures that if a key is required as part of a distance pair, that the sprite selection makes
        # sure a key is in that group.
        for dungeon_sprite in dungeon_room_sprite_group:
            if dungeon_sprite.distance_from_midpoint in distance_map:
                continue

            # Find all normal replacements. Make sure to include the sprite already present.
            possible_matches: list[SpriteId] = [
                it
                for it in list(choices)
                if is_compatible(
                    source=dungeon_sprite.sprite_id,
                    target=it,
                    placement=placement,
                    has_key=dungeon_sprite.has_key(),
                )
            ]

            if len(possible_matches) == 0:
                distance_map[
                    dungeon_sprite.distance_from_midpoint
                ] = dungeon_sprite.sprite_id
                continue

            weights: List[float] = get_weights(
                balancing=context.underworld_balancing,
                context=context,
                sprite_ids=possible_matches,
            )
            distance_map[dungeon_sprite.distance_from_midpoint] = random.choices(
                population=possible_matches,
                weights=weights,
            )[0]
    return distance_map


def reroll_underworld_enemies(context: Model) -> None:
    random = Random()
    random.seed(a=context.seed)

    for dungeon_room in context.underworld_rooms.values():
        if dungeon_room.id in _block_list:
            # Ignore rooms that are problematic (for example, kill room logic isn't working)
            continue

        # Randomize using Entities that occur anywhere in that Dungeon Room.
        if any(
            it for it in dungeon_room.sprites if it.sprite_id.role == SpriteType.BOSS
        ):
            # Skip all boss rooms, we shouldn't try to reroll those through this option.
            continue

        choices: Set[SpriteId] = context.underworld_choices[dungeon_room.spriteset_id]
        if len(choices) < 1:
            # Skip if there is nothing to switch.
            continue

        placement: Placement = (
            Placement.KILL_ROOM
            if dungeon_room.tag1.is_kill_room() or dungeon_room.tag2.is_kill_room()
            else Placement.ROOM
        )
        dungeon_sprites_by_role: Dict[SpriteType, List[UnderworldSprite]] = {
            it: list() for it in list(SpriteType)
        }
        for dungeon_sprite in dungeon_room.sprites:
            if dungeon_sprite.sprite_id.configuration().can_shuffle_in_room:
                dungeon_sprites_by_role[dungeon_sprite.sprite_id.role].append(
                    dungeon_sprite
                )

        for dungeon_sprites in dungeon_sprites_by_role.values():
            distance_map: Dict[int, SpriteId] = _generate_sprite_selections(
                context=context,
                random=random,
                dungeon_room_sprites=dungeon_sprites,
                choices=choices,
                placement=placement,
            )

            for dungeon_sprite in dungeon_sprites:
                next_sprite_id: SpriteId = distance_map[
                    dungeon_sprite.distance_from_midpoint
                ]
                if next_sprite_id != dungeon_sprite.sprite_id:
                    # Clear aux data because it may be unpredictable.
                    dungeon_sprite.aux0 = 0
                    dungeon_sprite.aux1 = 0
                dungeon_sprite.sprite_id = next_sprite_id
