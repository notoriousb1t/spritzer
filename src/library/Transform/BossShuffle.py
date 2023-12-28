from .Context import Context
from ..Model.DungeonRoomId import DungeonRoomId


def reroll_dungeon_bosses(context: Context) -> None:
    armos = context.dungeon_rooms[DungeonRoomId.xC8_EASTERN_PALACE_ARMOS_KNIGHTS_BOSS]
    trap = context.dungeon_rooms[DungeonRoomId.xA8_EASTERN_PALACE_STALFOS_SPAWN_ROOM]
