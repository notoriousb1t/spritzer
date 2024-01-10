from library.Model.UnderworldRoomId import UnderworldRoomId
from library.Transform.Context import Context


def reroll_underworld_bosses(context: Context) -> None:
    armos = context.underworld_rooms[UnderworldRoomId.xC8_EASTERN_PALACE_ARMOS_KNIGHTS_BOSS]
    trap = context.underworld_rooms[UnderworldRoomId.xA8_EASTERN_PALACE_STALFOS_SPAWN_ROOM]
