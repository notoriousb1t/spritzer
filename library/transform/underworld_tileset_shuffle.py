from random import Random
from typing import Dict, List
from library.model.blockset_id import BlocksetId
from library.model.sprite_type import SpriteType
from library.model.underworld_group import (
    UnderworldGroup,
    UnderworldGroupId,
    get_underworld_groups,
)
from library.model.underworld_room import UnderworldRoom
from library.model.model import Model


def reroll_underworld_blocksets(context: Model) -> None:
    random = Random()
    random.seed(a=context.seed)

    dungeon_room_groups: Dict[
        UnderworldGroupId, UnderworldGroup
    ] = get_underworld_groups()

    for dungeon_group in dungeon_room_groups.values():
        if len(dungeon_group.rooms) < 1:
            continue
        if len(dungeon_group.blocksets) < 2:
            continue

        matching_blocksets: List[BlocksetId] = dungeon_group.blocksets
        if len(matching_blocksets):
            continue

        next_blockset_id: BlocksetId = random.choice(seq=matching_blocksets)
        for room_id in dungeon_group.rooms:
            room: UnderworldRoom = context.underworld_rooms[room_id]
            if (
                any(
                    it
                    for it in room.sprites
                    if it.sprite_id.configuration().role in [SpriteType.BOSS, SpriteType.NPC]
                )
                or room_id in dungeon_group.exclude_from_tile_shuffle
            ):
                continue
            room.blockset_id = next_blockset_id
