from enum import IntEnum


class DungeonTag(IntEnum):
    """Tags used to indicate conditions for a Dungeon Room such as key prize or shutter conditions."""

    x0_NOTHING = 0
    x1_NW_KILL_ENEMY_TO_OPEN = 1
    x2_NE_KILL_ENEMY_TO_OPEN = 2
    x3_SW_KILL_ENEMY_TO_OPEN = 3
    x4_SE_KILL_ENEMY_TO_OPEN = 4
    x5_W_KILL_ENEMY_TO_OPEN = 5
    x6_E_KILL_ENEMY_TO_OPEN = 6
    x7_N_KILL_ENEMY_TO_OPEN = 7
    x8_S_KILL_ENEMY_TO_OPEN = 8
    x9_CLEAR_QUADRANT_TO_OPEN = 9
    xA_CLEAR_ROOM_TO_OPEN = 10
    xB_NW_MOVE_BLOCK_TO_OPEN = 11
    xC_NE_MOVE_BLOCK_TO_OPEN = 12
    xD_SW_MOVE_BLOCK_TO_OPEN = 13
    xE_SE_MOVE_BLOCK_TO_OPEN = 14
    xF_W_MOVE_BLOCK_TO_OPEN = 15
    x10_E_MOVE_BLOCK_TO_OPEN = 16
    x11_N_MOVE_BLOCK_TO_OPEN = 17
    x12_S_MOVE_BLOCK_TO_OPEN = 18
    x13_MOVE_BLOCK_TO_OPEN = 19
    x14_PULL_LEVER_TO_OPEN = 20
    x15_CLEAR_LEVEL_TO_OPEN_DOOR = 21
    x16_SWITCH_OPENS_DOOR_HOLD = 22
    x17_SWITCH_OPENS_DOOR_TOGGLE = 23
    x18_TURN_OFF_WATER = 24
    x19_TURN_ON_WATER = 25
    x1A_WATER_GATE = 26
    x1B_WATER_TWIN = 27
    x1C_SECRET_WALL_RIGHT = 28
    x1D_SECRET_WALL_LEFT = 29
    x1E_CRASH_1 = 30
    x1F_CRASH_2 = 31
    x20_USE_SWITCH_TO_BOMB_WALL = 32
    x21_HOLES_0 = 33
    x22_OPEN_CHEST_FOR_HOLES_0 = 34
    x23_HOLES_1 = 35
    x24_HOLES_2 = 36
    x25_KILL_ENEMY_TO_CLEAR_LEVEL = 37
    x26_SE_KILL_ENEMY_TO_MOVE_BLOCK = 38
    x27_TRIGGER_ACTIVATED_CHEST = 39
    x28_USE_LEVER_TO_BOMB_WALL = 40
    x29_NW_KILL_ENEMY_FOR_CHEST = 41
    x2A_NE_KILL_ENEMY_FOR_CHEST = 42
    x2B_SW_KILL_ENEMIES_FOR_CHEST = 43
    x2C_SE_KILL_ENEMY_FOR_CHEST = 44
    x2D_W_KILL_ENEMY_FOR_CHEST = 45
    x2E_E_KILL_ENEMY_FOR_CHEST = 46
    x2F_N_KILL_ENEMY_FOR_CHEST = 47
    x30_S_KILL_ENEMY_FOR_CHEST = 48
    x31_CLEAR_QUADRANT_FOR_CHEST = 49
    x32_CLEAR_ROOM_FOR_CHEST = 50
    x33_LIGHT_TORCHES_TO_OPEN = 51
    x34_HOLES_3 = 52
    x35_HOLES_4 = 53
    x36_HOLES_5 = 54
    x37_HOLES_6 = 55
    x38_AGAHNIMS_ROOM = 56
    x39_HOLES_7 = 57
    x3A_HOLES_8 = 58
    x3B_OPEN_CHEST_FOR_HOLES_8 = 59
    x3C_MOVE_BLOCK_TO_GET_CHEST = 60
    x3D_KILL_TO_OPEN_GANONS_DOOR = 61
    x3E_LIGHT_TORCHES_TO_GET_CHEST = 62
    x3F_KILL_BOSS_AGAIN = 63

    def __str__(self) -> str:
        return self.name

    def is_kill_room(self) -> bool:
        return self in [
            DungeonTag.x27_TRIGGER_ACTIVATED_CHEST,
            DungeonTag.x1_NW_KILL_ENEMY_TO_OPEN,
            DungeonTag.x2_NE_KILL_ENEMY_TO_OPEN,
            DungeonTag.x3_SW_KILL_ENEMY_TO_OPEN,
            DungeonTag.x4_SE_KILL_ENEMY_TO_OPEN,
            DungeonTag.x5_W_KILL_ENEMY_TO_OPEN,
            DungeonTag.x6_E_KILL_ENEMY_TO_OPEN,
            DungeonTag.x7_N_KILL_ENEMY_TO_OPEN,
            DungeonTag.x8_S_KILL_ENEMY_TO_OPEN,
            DungeonTag.x9_CLEAR_QUADRANT_TO_OPEN,
            DungeonTag.xA_CLEAR_ROOM_TO_OPEN,
            DungeonTag.x15_CLEAR_LEVEL_TO_OPEN_DOOR,
            DungeonTag.x25_KILL_ENEMY_TO_CLEAR_LEVEL,
            DungeonTag.x26_SE_KILL_ENEMY_TO_MOVE_BLOCK,
            DungeonTag.x29_NW_KILL_ENEMY_FOR_CHEST,
            DungeonTag.x2A_NE_KILL_ENEMY_FOR_CHEST,
            DungeonTag.x2B_SW_KILL_ENEMIES_FOR_CHEST,
            DungeonTag.x2C_SE_KILL_ENEMY_FOR_CHEST,
            DungeonTag.x2D_W_KILL_ENEMY_FOR_CHEST,
            DungeonTag.x2E_E_KILL_ENEMY_FOR_CHEST,
            DungeonTag.x2F_N_KILL_ENEMY_FOR_CHEST,
            DungeonTag.x30_S_KILL_ENEMY_FOR_CHEST,
            DungeonTag.x31_CLEAR_QUADRANT_FOR_CHEST,
            DungeonTag.x32_CLEAR_ROOM_FOR_CHEST,
            DungeonTag.x38_AGAHNIMS_ROOM,
            DungeonTag.x3D_KILL_TO_OPEN_GANONS_DOOR,
            DungeonTag.x3F_KILL_BOSS_AGAIN,
        ]
