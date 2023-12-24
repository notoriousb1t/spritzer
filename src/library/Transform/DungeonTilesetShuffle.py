from typing import Dict, List

from ..Model import DungeonRoom, DungeonRoomTilesetId
from .Context import Context


_caves = [
    DungeonRoomTilesetId.xD_TURTLE_ROCK,
    DungeonRoomTilesetId.x12_FAIRY_CAVE,
    DungeonRoomTilesetId.x14_CAVE,
]
_houses_and_shops = [
    DungeonRoomTilesetId.xF_SAHASRAHLA,
    DungeonRoomTilesetId.x3_HOUSE,
    DungeonRoomTilesetId.x4_SANTUARY,
    DungeonRoomTilesetId.xA_THIEVES_TOWN,
    DungeonRoomTilesetId.x11_SHOP,
    DungeonRoomTilesetId.x10_LINKS_HOUSE,
]
_dungeons = [
    DungeonRoomTilesetId.x0_CASTLE,
    DungeonRoomTilesetId.x1_DUNGEON,
    DungeonRoomTilesetId.x2_AGAHNIM,
    DungeonRoomTilesetId.x5_EASTERN_PALACE,
    DungeonRoomTilesetId.x6_TOWER1,
    DungeonRoomTilesetId.x7_TOWER_HERA,
    DungeonRoomTilesetId.x8_SWAMP_PALACE,
    DungeonRoomTilesetId.x9_SKULL_WOODS,
    DungeonRoomTilesetId.xB_ICE_PALACE,
    DungeonRoomTilesetId.xC_MISERY_MIRE,
    DungeonRoomTilesetId.xE_DESERT_PALACE,
    DungeonRoomTilesetId.x13_GANON,
]
_tileset_lists = [
    _caves,
    _houses_and_shops,
    _dungeons,
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
        for room in room_list:
            room.tileset_id = next_tileset_id
