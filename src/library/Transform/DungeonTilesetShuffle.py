from typing import Dict, List

from ..Model.DungeonRoomId import DungeonRoomId

from ..Model import DungeonRoom, DungeonRoomTilesetId
from .Context import Context

_caves = [
    DungeonRoomTilesetId.xD_TURTLE_ROCK,
    DungeonRoomTilesetId.x12_FAIRY_CAVE,
    DungeonRoomTilesetId.x14_CAVE,
    DungeonRoomTilesetId.x1_DUNGEON,
]
_dungeons1 = [
    DungeonRoomTilesetId.x0_CASTLE,
    DungeonRoomTilesetId.x5_EASTERN_PALACE,
    DungeonRoomTilesetId.xE_DESERT_PALACE,
    DungeonRoomTilesetId.xB_ICE_PALACE,
]
_dungeons2 = [
    DungeonRoomTilesetId.x6_TOWER1,
    DungeonRoomTilesetId.x7_TOWER_HERA,
    DungeonRoomTilesetId.x8_SWAMP_PALACE,
    DungeonRoomTilesetId.x9_SKULL_WOODS,
    DungeonRoomTilesetId.xC_MISERY_MIRE,
    DungeonRoomTilesetId.x13_GANON,
]
_tileset_lists = [
    _caves,
    _dungeons1,
    _dungeons2,
]
_blocklist = [
    # Requires special tiles for jumping between layers.
    DungeonRoomId.x2A_PALACE_OF_DARKNESS_BIG_HUB_ROOM,
]


def reroll_dungeon_tilesets(context: Context) -> None:
    tileset_dict: Dict[DungeonRoomTilesetId, List[DungeonRoom]] = {
        it: list() for it in list(DungeonRoomTilesetId)
    }
    for room in context.dungeon_rooms.values():
        tileset_dict[room.tileset_id].append(room)

    for tileset_id, room_list in tileset_dict.items():
        if len(room_list) == 0:
            continue

        matching_tilesets = next(
            (it for it in _tileset_lists if tileset_id in it), None
        )
        if matching_tilesets == None:
            continue

        next_tileset_id = context.random.choice(matching_tilesets)
        for room in [it for it in room_list if not it in _blocklist]:
            room.tileset_id = next_tileset_id
