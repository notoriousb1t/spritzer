from ..Model.DungeonRoomGroup import get_dungeon_room_groups
from .Context import Context


def reroll_dungeon_tilesets(context: Context) -> None:
    dungeon_room_groups = get_dungeon_room_groups()

    for dungeon_group in dungeon_room_groups.values():
        if len(dungeon_group.rooms) < 1:
            continue
        if len(dungeon_group.tilesets) < 2:
            continue

        matching_tilesets = dungeon_group.tilesets
        if matching_tilesets == None:
            continue

        next_tileset_id = context.random.choice(matching_tilesets)
        for room in dungeon_group.rooms:
            room.tileset_id = next_tileset_id
