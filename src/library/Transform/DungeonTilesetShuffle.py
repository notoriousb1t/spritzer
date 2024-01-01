from ..Model import get_dungeon_room_groups, SpriteType
from .Context import Context


def reroll_dungeon_blocksets(context: Context) -> None:
    dungeon_room_groups = get_dungeon_room_groups()

    for dungeon_group in dungeon_room_groups.values():
        if len(dungeon_group.rooms) < 1:
            continue
        if len(dungeon_group.blocksets) < 2:
            continue

        matching_blocksets = dungeon_group.blocksets
        if matching_blocksets == None:
            continue

        next_blockset_id = context.random.choice(matching_blocksets)
        for room_id in dungeon_group.rooms:
            room = context.dungeon_rooms[room_id]
            if (
                any(
                    it
                    for it in room.dungeon_sprites
                    if it.sprite_id.meta().role in [SpriteType.BOSS, SpriteType.NPC]
                )
                or room_id in dungeon_group.exclude_from_tile_shuffle
            ):
                continue
            room.blockset_id = next_blockset_id
