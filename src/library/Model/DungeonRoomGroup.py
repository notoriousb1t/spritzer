from attr import dataclass
from enum import Enum, auto
from typing import Dict, List
from .DungeonRoomId import DungeonRoomId
from .DungeonRoomTilesetId import DungeonRoomTilesetId
from .SpriteBlockset import SpriteBlockset


class DungeonRoomGroupId(Enum):
    # Beginning
    HYRULE_CASTLE_ENTRANCE = auto()
    HYRULE_CASTLE_DUNGEON = auto()
    HYRULE_CASTLE_ESCAPE = auto()
    # First Part
    SANCTUARY = auto()
    DESERT_PALACE_FRONT = auto()
    DESERT_PALACE_BACK = auto()
    EASTERN_PALACE = auto()
    TOWER_OF_HERA = auto()
    AGAHNIMS_TOWER = auto()
    # Second Part
    PALACE_OF_DARKNESS = auto()
    SWAMP_PALACE = auto()
    SKULL_WOODS = auto()
    ICE_PALACE = auto()
    THIEVES_TOWN_ENTRANCE = auto()
    THIEVES_TOWN_BASEMENT = auto()
    THIEVES_TOWN_ATTIC = auto()
    MISERY_MIRE = auto()
    TURTLE_ROCK = auto()
    # End-game
    GANONS_TOWER = auto()

    def __str__(self) -> str:
        return self.name


@dataclass
class DungeonRoomGroup:
    """A group of Dungeon Rooms. This may be related rooms or an entire dungeon."""

    rooms: List[DungeonRoomId] = []
    tilesets: List[DungeonRoomTilesetId] = []


def get_dungeon_room_groups() -> Dict[DungeonRoomGroupId, DungeonRoomGroup]:
    return {
        DungeonRoomGroupId.HYRULE_CASTLE_ENTRANCE: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x01_HYRULE_CASTLE_NORTH_CORRIDOR,
                DungeonRoomId.x50_HYRULE_CASTLE_WEST_CORRIDOR,
                DungeonRoomId.x51_HYRULE_CASTLE_THRONE_ROOM,
                DungeonRoomId.x52_HYRULE_CASTLE_EAST_CORRIDOR,
                DungeonRoomId.x60_HYRULE_CASTLE_WEST_ENTRANCE_ROOM,
                DungeonRoomId.x61_HYRULE_CASTLE_MAIN_ENTRANCE_ROOM,
                DungeonRoomId.x62_HYRULE_CASTLE_EAST_ENTRANCE_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.HYRULE_CASTLE_DUNGEON: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x41_HYRULE_CASTLE_FIRST_DARK_ROOM,
                DungeonRoomId.x55_CASTLE_SECRET_ENTRANCE_UNCLE_DEATH_ROOM,
                DungeonRoomId.x70_HYRULE_CASTLE_SMALL_CORRIDOR_TO_JAIL_CELLS,
                DungeonRoomId.x71_HYRULE_CASTLE_BOOMERANG_CHEST_ROOM,
                DungeonRoomId.x72_HYRULE_CASTLE_MAP_CHEST_ROOM,
                DungeonRoomId.x80_HYRULE_CASTLE_JAIL_CELL_ROOM,
                DungeonRoomId.x81_HYRULE_CASTLE_NEXT_TO_CHASM_ROOM,
                DungeonRoomId.x82_HYRULE_CASTLE_BASEMENT_CHASM_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.HYRULE_CASTLE_ESCAPE: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x11_HYRULE_CASTLE_BOMBABLE_STOCK_ROOM,
                DungeonRoomId.x21_HYRULE_CASTLE_KEY_RAT_ROOM,
                DungeonRoomId.x22_HYRULE_CASTLE_SEWER_TEXT_TRIGGER_ROOM,
                DungeonRoomId.x32_HYRULE_CASTLE_SEWER_KEY_CHEST_ROOM,
                DungeonRoomId.x42_HYRULE_CASTLE_6_ROPES_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.SANCTUARY: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x02_HYRULE_CASTLE_SWITCH_ROOM,
                DungeonRoomId.x12_SANCTUARY,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.DESERT_PALACE_FRONT: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x73_DESERT_PALACE_BIG_CHEST_ROOM,
                DungeonRoomId.x74_DESERT_PALACE_MAP_CHEST_ROOM,
                DungeonRoomId.x75_DESERT_PALACE_BIG_KEY_CHEST_ROOM,
                DungeonRoomId.x83_DESERT_PALACE_WEST_ENTRANCE_ROOM,
                DungeonRoomId.x84_DESERT_PALACE_MAIN_ENTRANCE_ROOM,
                DungeonRoomId.x85_DESERT_PALACE_EAST_ENTRANCE_ROOM,
            ],
            tilesets=[
                DungeonRoomTilesetId.x5_EASTERN_PALACE,
                DungeonRoomTilesetId.x7_TOWER_HERA,
                DungeonRoomTilesetId.xA_THIEVES_TOWN,
                DungeonRoomTilesetId.xE_DESERT_PALACE,
                DungeonRoomTilesetId.x12_FAIRY_CAVE,
            ],
        ),
        DungeonRoomGroupId.DESERT_PALACE_FRONT: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x63_DESERT_PALACE_FINAL_SECTION_ENTRANCE_ROOM,
                DungeonRoomId.x53_DESERT_PALACE_POPOS_2_BEAMOS_HELLWAY_ROOM,
                DungeonRoomId.x43_DESERT_PALACE_TORCH_PUZZLE_MOVING_WALL_ROOM,
                DungeonRoomId.x33_DESERT_PALACE_LANMOLAS_BOSS,
            ],
            tilesets=[
                DungeonRoomTilesetId.x5_EASTERN_PALACE,
                DungeonRoomTilesetId.x7_TOWER_HERA,
                DungeonRoomTilesetId.xA_THIEVES_TOWN,
                DungeonRoomTilesetId.xE_DESERT_PALACE,
                DungeonRoomTilesetId.x12_FAIRY_CAVE,
            ],
        ),
        DungeonRoomGroupId.EASTERN_PALACE: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x89_EASTERN_PALACE_FAIRY_ROOM,
                DungeonRoomId.x99_EASTERN_PALACE_EYEGORE_KEY_ROOM,
                DungeonRoomId.xA8_EASTERN_PALACE_STALFOS_SPAWN_ROOM,
                DungeonRoomId.xA9_EASTERN_PALACE_BIG_CHEST_ROOM,
                DungeonRoomId.xAA_EASTERN_PALACE_MAP_CHEST_ROOM,
                DungeonRoomId.xB8_EASTERN_PALACE_BIG_KEY_ROOM,
                DungeonRoomId.xB9_EASTERN_PALACE_LOBBY_CANNONBALLS_ROOM,
                DungeonRoomId.xBA_EASTERN_PALACE_DARK_ANTIFAIRY_KEY_POT_ROOM,
                DungeonRoomId.xC8_EASTERN_PALACE_ARMOS_KNIGHTS_BOSS,
                DungeonRoomId.xC9_EASTERN_PALACE_ENTRANCE_ROOM,
                DungeonRoomId.xD8_EASTERN_PALACE_ZELDAGAMER_ROOM_PRE_ARMOS_KNIGHTS_ROOM,
                DungeonRoomId.xD9_EASTERN_PALACE_CANONBALL_ROOM,
                DungeonRoomId.xDA_EASTERN_PALACE_2_BUBBLE_WITH_SWITCH_UNDER_POT,
            ],
            tilesets=[
                DungeonRoomTilesetId.x0_CASTLE,
                DungeonRoomTilesetId.x1_DUNGEON,
                DungeonRoomTilesetId.x2_AGAHNIM,
                DungeonRoomTilesetId.x4_SANTUARY,
                DungeonRoomTilesetId.x5_EASTERN_PALACE,
                DungeonRoomTilesetId.x6_TOWER1,
                DungeonRoomTilesetId.x7_TOWER_HERA,
                DungeonRoomTilesetId.x8_SWAMP_PALACE,
                DungeonRoomTilesetId.x9_SKULL_WOODS,
                DungeonRoomTilesetId.xA_THIEVES_TOWN,
                DungeonRoomTilesetId.xB_ICE_PALACE,
                DungeonRoomTilesetId.xC_MISERY_MIRE,
                DungeonRoomTilesetId.xD_TURTLE_ROCK,
                DungeonRoomTilesetId.xE_DESERT_PALACE,
                DungeonRoomTilesetId.xF_SAHASRAHLA,
                DungeonRoomTilesetId.x14_CAVE,
            ],
        ),
        DungeonRoomGroupId.TOWER_OF_HERA: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x07_TOWER_OF_HERA_MOLDORM_BOSS,
                DungeonRoomId.x17_TOWER_OF_HERA_MOLDORM_FALL_ROOM,
                DungeonRoomId.x27_TOWER_OF_HERA_BIG_CHEST,
                DungeonRoomId.x31_TOWER_OF_HERA_HARDHAT_BEETLES_ROOM,
                DungeonRoomId.x77_TOWER_OF_HERA_ENTRANCE_ROOM,
                DungeonRoomId.x87_TOWER_OF_HERA_TILE_ROOM,
                DungeonRoomId.xA7_TOWER_OF_HERA_FAIRY_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.AGAHNIMS_TOWER: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x20_AGAHNIM_S_TOWER_AGAHNIM_BOSS,
                DungeonRoomId.x30_AGAHNIM_S_TOWER_MAIDEN_SACRIFICE_CHAMBER,
                DungeonRoomId.x40_AGAHNIM_S_TOWER_FINAL_BRIDGE_ROOM,
                DungeonRoomId.xB0_AGAHNIM_S_TOWER_CIRCLE_OF_POTS,
                DungeonRoomId.xC0_AGAHNIM_S_TOWER_DARK_BRIDGE_ROOM,
                DungeonRoomId.xD0_AGAHNIM_S_TOWER_DARK_MAZE,
                DungeonRoomId.xE0_AGAHNIM_S_TOWER_ENTRANCE_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.PALACE_OF_DARKNESS: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x09_PALACE_OF_DARKNESS,
                DungeonRoomId.x0A_PALACE_OF_DARKNESS_STALFOS_TRAP_ROOM,
                DungeonRoomId.x0B_PALACE_OF_DARKNESS_TURTLE_ROOM,
                DungeonRoomId.x19_PALACE_OF_DARKNESS_DARK_MAZE,
                DungeonRoomId.x1A_PALACE_OF_DARKNESS_BIG_CHEST_ROOM,
                DungeonRoomId.x1B_PALACE_OF_DARKNESS_MIMICS_MOVING_WALL_ROOM,
                DungeonRoomId.x2A_PALACE_OF_DARKNESS_BIG_HUB_ROOM,
                DungeonRoomId.x2B_PALACE_OF_DARKNESS_MAP_CHEST_FAIRY_ROOM,
                DungeonRoomId.x3A_PALACE_OF_DARKNESS_BOMBABLE_FLOOR_ROOM,
                DungeonRoomId.x3B_PALACE_OF_DARKNESS_SPIKE_BLOCK_CONVEYOR_ROOM,
                DungeonRoomId.x4A_PALACE_OF_DARKNESS_ENTRANCE_ROOM,
                DungeonRoomId.x4B_PALACE_OF_DARKNESS_WARPS_SOUTH_MIMICS_ROOM,
                DungeonRoomId.x5A_PALACE_OF_DARKNESS_HELMASAUR_KING_BOSS,
                DungeonRoomId.x6A_PALACE_OF_DARKNESS_RUPEE_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.SWAMP_PALACE: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x06_SWAMP_PALACE_ARRGHUS_BOSS,
                DungeonRoomId.x16_SWAMP_PALACE_SWIMMING_TREADMILL,
                DungeonRoomId.x26_SWAMP_PALACE_STATUE_ROOM,
                DungeonRoomId.x28_SWAMP_PALACE_ENTRANCE_ROOM,
                DungeonRoomId.x34_SWAMP_PALACE_PUSH_BLOCK_PUZZLE_PRE_BIG_KEY_ROOM,
                DungeonRoomId.x35_SWAMP_PALACE_BIG_KEY_BS_ROOM,
                DungeonRoomId.x36_SWAMP_PALACE_BIG_CHEST_ROOM,
                DungeonRoomId.x37_SWAMP_PALACE_MAP_CHEST_WATER_FILL_ROOM,
                DungeonRoomId.x38_SWAMP_PALACE_KEY_POT_ROOM,
                DungeonRoomId.x46_SWAMP_PALACE_COMPASS_CHEST_ROOM,
                DungeonRoomId.x54_SWAMP_PALACE_UPSTAIRS_PITS_ROOM,
                DungeonRoomId.x66_SWAMP_PALACE_HIDDEN_CHEST_HIDDEN_DOOR_ROOM,
                DungeonRoomId.x76_SWAMP_PALACE_WATER_DRAIN_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.SKULL_WOODS: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x29_SKULL_WOODS_MOTHULA_BOSS,
                DungeonRoomId.x39_SKULL_WOODS_GIBDO_KEY_MOTHULA_HOLE_ROOM,
                DungeonRoomId.x49_SKULL_WOODS_GIBDO_TORCH_PUZZLE_ROOM,
                DungeonRoomId.x56_SKULL_WOODS_KEY_POT_TRAP_ROOM,
                DungeonRoomId.x57_SKULL_WOODS_BIG_KEY_ROOM,
                DungeonRoomId.x58_SKULL_WOODS_BIG_CHEST_ROOM,
                DungeonRoomId.x59_SKULL_WOODS_FINAL_SECTION_ENTRANCE_ROOM,
                DungeonRoomId.x67_SKULL_WOODS_COMPASS_CHEST_ROOM,
                DungeonRoomId.x68_SKULL_WOODS_KEY_CHEST_TRAP_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.ICE_PALACE: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x0E_ICE_PALACE_ENTRANCE_ROOM,
                DungeonRoomId.x1E_ICE_PALACE_BOMB_FLOOR_BARI_ROOM,
                DungeonRoomId.x1F_ICE_PALACE_PENGATOR_BIG_KEY_ROOM,
                DungeonRoomId.x2E_ICE_PALACE_COMPASS_ROOM,
                DungeonRoomId.x3E_ICE_PALACE_STALFOS_KNIGHTS_CONVEYOR_HELLWAY,
                DungeonRoomId.x3F_ICE_PALACE_MAP_CHEST_ROOM,
                DungeonRoomId.x4E_ICE_PALACE_BOMB_JUMP_ROOM,
                DungeonRoomId.x4F_ICE_PALACE_CLONE_ROOM_FAIRY_ROOM,
                DungeonRoomId.x5E_ICE_PALACE_LONELY_FIREBAR,
                DungeonRoomId.x5F_ICE_PALACE_HIDDEN_CHEST_SPIKE_FLOOR_ROOM,
                DungeonRoomId.x6E_ICE_PALACE_PENGATORS_ROOM,
                DungeonRoomId.x7E_ICE_PALACE_HIDDEN_CHEST_BOMBABLE_FLOOR_ROOM,
                DungeonRoomId.x7F_ICE_PALACE_BIG_SPIKE_TRAPS_ROOM,
                DungeonRoomId.x8E_ICE_PALACE_BLOBS_WITH_TETRIS_BARRIER,
                DungeonRoomId.x9E_ICE_PALACE_BIG_CHEST_ROOM,
                DungeonRoomId.x9F_ICE_PALACE_ROOM_WITH_ICE_FLOOR_KEY_AND_4_WALL_RATS,
                DungeonRoomId.xAE_ICE_PALACE_2_BLUE_BARI_AND_HIDDEN_CHEST,
                DungeonRoomId.xAF_ICE_PALACE_ICE_BRIDGE_ROOM,
                DungeonRoomId.xBE_ICE_PALACE_BLOCK_PUZZLE_ROOM,
                DungeonRoomId.xBF_ICE_PALACE_CLONE_ROOM_SWITCH_ROOM,
                DungeonRoomId.xCE_ICE_PALACE_HOLE_TO_KHOLDSTARE_ROOM,
                DungeonRoomId.xDE_ICE_PALACE_KHOLDSTARE_BOSS,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.MISERY_MIRE: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x90_MISERY_MIRE_VITREOUS_BOSS,
                DungeonRoomId.x91_MISERY_MIRE_FINAL_SWITCH_ROOM,
                DungeonRoomId.x92_MISERY_MIRE_DARK_BOMB_WALL_SWITCHES_ROOM,
                DungeonRoomId.x93_MISERY_MIRE_DARK_CANE_FLOOR_SWITCH_PUZZLE_ROOM,
                DungeonRoomId.x97_MISERY_MIRE_TORCH_PUZZLE_MOVING_WALL_ROOM,
                DungeonRoomId.x98_MISERY_MIRE_ENTRANCE_ROOM,
                DungeonRoomId.xA0_MISERY_MIRE_PRE_VITREOUS_ROOM,
                DungeonRoomId.xA1_MISERY_MIRE_FISH_ROOM,
                DungeonRoomId.xA2_MISERY_MIRE_BRIDGE_KEY_CHEST_ROOM,
                DungeonRoomId.xA3_MISERY_MIRE_EMPTY_L_CONNECTING_ROOM,
                DungeonRoomId.xB1_MISERY_MIRE_HOURGLASS_ROOM,
                DungeonRoomId.xB2_MISERY_MIRE_SLUG_ROOM,
                DungeonRoomId.xB3_MISERY_MIRE_SPIKE_KEY_CHEST_ROOM,
                DungeonRoomId.xC1_MISERY_MIRE_COMPASS_CHEST_TILE_ROOM,
                DungeonRoomId.xC2_MISERY_MIRE_BIG_HUB_ROOM,
                DungeonRoomId.xC3_MISERY_MIRE_BIG_CHEST_ROOM,
                DungeonRoomId.xD1_MISERY_MIRE_CONVEYOR_SLUG_BIG_KEY_ROOM,
                DungeonRoomId.xD2_MISERY_MIRE_MIRE02_WIZZROBES_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.THIEVES_TOWN_ENTRANCE: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.xAC_THIEVES_TOWN_BLIND_THE_THIEF_BOSS,
                DungeonRoomId.xCB_THIEVES_TOWN_NORTH_WEST_ENTRANCE_ROOM,
                DungeonRoomId.xCC_THIEVES_TOWN_NORTH_EAST_ENTRANCE_ROOM,
                DungeonRoomId.xDB_THIEVES_TOWN_MAIN_SOUTH_WEST_ENTRANCE_ROOM,
                DungeonRoomId.xDC_THIEVES_TOWN_SOUTH_EAST_ENTRANCE_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.THIEVES_TOWN_ATTIC: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x64_THIEVES_TOWN_WEST_ATTIC_ROOM,
                DungeonRoomId.x65_THIEVES_TOWN_EAST_ATTIC_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.THIEVES_TOWN_BASEMENT: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x44_THIEVES_TOWN_BIG_CHEST_ROOM,
                DungeonRoomId.x45_THIEVES_TOWN_JAIL_CELLS_ROOM,
                DungeonRoomId.xAB_THIEVES_TOWN_MOVING_SPIKES_KEY_POT_ROOM,
                DungeonRoomId.xBB_THIEVES_TOWN_HELLWAY,
                DungeonRoomId.xBC_THIEVES_TOWN_CONVEYOR_TOILET,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.TURTLE_ROCK: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x04_TURTLE_ROCK_CRYSTA_ROLLER_ROOM,
                DungeonRoomId.x13_TURTLE_ROCK_HOKKU_BOKKU_KEY_ROOM_2,
                DungeonRoomId.x14_TURTLE_ROCK_BIG_KEY_ROOM,
                DungeonRoomId.x15_TURTLE_ROCK,
                DungeonRoomId.x23_TURTLE_ROCK_WEST_EXIT_TO_BALCONY,
                DungeonRoomId.x24_TURTLE_ROCK_DOUBLE_HOKKU_BOKKU_BIG_CHEST_ROOM,
                DungeonRoomId.xA4_TURTLE_ROCK_TRINEXX_BOSS,
                DungeonRoomId.xB4_TURTLE_ROCK_PRE_TRINEXX_ROOM,
                DungeonRoomId.xB5_TURTLE_ROCK_DARK_MAZE,
                DungeonRoomId.xB6_TURTLE_ROCK_CHAIN_CHOMPS_ROOM,
                DungeonRoomId.xB7_TURTLE_ROCK_MAP_CHEST_KEY_CHEST_ROLLER_ROOM,
                DungeonRoomId.xC4_TURTLE_ROCK_FINAL_CRYSTAL_SWITCH_PUZZLE_ROOM,
                DungeonRoomId.xC5_TURTLE_ROCK_LASER_BRIDGE,
                DungeonRoomId.xC6_TURTLE_ROCK_AFTER_ENTRANCE_SQUARE_SOMARIA_PLATFORM_ROOM,
                DungeonRoomId.xC7_TURTLE_ROCK_TORCH_PUZZLE,
                DungeonRoomId.xD5_TURTLE_ROCK_LASER_KEY_ROOM,
                DungeonRoomId.xD6_TURTLE_ROCK_ENTRANCE_ROOM,
            ],
            tilesets=[],
        ),
        DungeonRoomGroupId.GANONS_TOWER: DungeonRoomGroup(
            rooms=[
                DungeonRoomId.x0C_GANON_S_TOWER_ENTRANCE_ROOM,
                DungeonRoomId.x0D_GANON_S_TOWER_AGAHNIM2_BOSS,
                DungeonRoomId.x1C_GANON_S_TOWER_ICE_ARMOS,
                DungeonRoomId.x1D_GANON_S_TOWER_FINAL_HALLWAY,
                DungeonRoomId.x3D_GANON_S_TOWER_TORCH_ROOM_2,
                DungeonRoomId.x4C_GANON_S_TOWER_MINI_HELMASAUR_CONVEYOR_ROOM,
                DungeonRoomId.x4D_GANON_S_TOWER_MOLDORM_ROOM,
                DungeonRoomId.x5B_GANON_S_TOWER_SPIKE_PIT_ROOM,
                DungeonRoomId.x5C_GANON_S_TOWER_GANON_BALL_Z,
                DungeonRoomId.x5D_GANON_S_TOWER_GAUNTLET_1_2_3,
                DungeonRoomId.x6B_GANON_S_TOWER_MIMICS_ROOMS,
                DungeonRoomId.x6C_GANON_S_TOWER_LANMOLAS_ROOM,
                DungeonRoomId.x6D_GANON_S_TOWER_GAUNTLET_4_5,
                DungeonRoomId.x7B_GANON_S_TOWER_SIDWAYS_CONVEYORS_4_CHEST_4_SHOOTER_SQUARE_PIT,
                DungeonRoomId.x7C_GANON_S_TOWER_EAST_SIDE_COLLAPSING_BRIDGE_EXPLODING_WALL_ROOM,
                DungeonRoomId.x7D_GANON_S_TOWER_WINDER_WARP_MAZE_ROOM,
                DungeonRoomId.x8B_GANON_S_TOWER_BLOCK_PUZZLE_SPIKE_SKIP_MAP_CHEST_ROOM,
                DungeonRoomId.x8C_GANON_S_TOWER_EAST_AND_WEST_DOWNSTAIRS_BIG_CHEST_ROOM,
                DungeonRoomId.x8D_GANON_S_TOWER_TILE_TORCH_PUZZLE_ROOM,
                DungeonRoomId.x95_GANON_S_TOWER_FINAL_COLLAPSING_BRIDGE_ROOM,
                DungeonRoomId.x96_GANON_S_TOWER_TORCHES_1_ROOM,
                DungeonRoomId.x9B_GANON_S_TOWER_MANY_SPIKES_WARP_MAZE_ROOM,
                DungeonRoomId.x9C_GANON_S_TOWER_INVISIBLE_FLOOR_MAZE_ROOM,
                DungeonRoomId.x9D_GANON_S_TOWER_COMPASS_CHEST_INVISIBLE_FLOOR_ROOM,
                DungeonRoomId.xA5_GANON_S_TOWER_WIZZROBES_ROOMS,
                DungeonRoomId.xA6_GANON_S_TOWER_MOLDORM_FALL_ROOM,
            ],
            tilesets=[],
        ),
    }


# List of remaining rooms to group.
_all_items = [
    DungeonRoomId.x03_HOULIHAN_ROOM,
    DungeonRoomId.x08_CAVE_HEALING_FAIRY,
    DungeonRoomId.x10_GANON_EVACUATION_ROUTE,
    DungeonRoomId.x18_BIG_FAIRY_DROP_ENTRANCE_CAVE,
    DungeonRoomId.x2C_HOOKSHOT_CAVE_BACKDOOR_BIG_FAIRY,
    DungeonRoomId.x2F_CAVE_KAKARIKO_WELL_HP,
    DungeonRoomId.x3C_HOOKSHOT_CAVE,
    DungeonRoomId.xDF__TOP_OF_BACKWARD_DEATH_MOUNTAIN_CAVE,
    DungeonRoomId.xE1_CAVE_LOST_WOODS_HP,
    DungeonRoomId.xE2_CAVE_LUMBERJACK_S_TREE_HP,
    DungeonRoomId.xE3_CAVE_1_2_MAGIC,
    DungeonRoomId.xE4_CAVE_LOST_OLD_MAN_HOUSE_CAVE,
    DungeonRoomId.xE5_CAVE_LOST_OLD_MAN_HOUSE_CAVE_BACK,
    DungeonRoomId.xE6_CAVE_WITH_A_BUNCH_OF_KEESE,
    DungeonRoomId.xE7_CAVE_WITH_A_BUNCH_OF_KEESE_2,
    DungeonRoomId.xE8_SUPER_BUNNY_CAVE_EXIT,
    DungeonRoomId.xEA_CAVE_INSIDE_SPECTACLE_ROCK_HP,
    DungeonRoomId.xEB_BUMPER_CAVE_ENTRANCE1,
    DungeonRoomId.xED_CAVE_NO_CLUE,
    DungeonRoomId.xEE_CAVE_SPIRAL_CAVE,
    DungeonRoomId.xEF_CAVE_CRYSTAL_SWITCH_5_CHESTS_ROOM,
    DungeonRoomId.xF0_CAVE_LOST_OLD_MAN_STARTING_CAVE_ENTRANCE,
    DungeonRoomId.xF1_CAVE_LOST_OLD_MAN_STARTING_CAVE_EXIT,
    DungeonRoomId.xF2_HOUSE_OLD_WOMAN_NEXT_DOOR,
    DungeonRoomId.xF3_HOUSE_OLD_WOMAN_SAHASRAHLA_S_WIFE,
    DungeonRoomId.xF4_HOUSE_ANGRY_BROTHERS_EXIT_TO_MAZE_GAME,
    DungeonRoomId.xF5_HOUSE_ANGRY_BROTHERS_ENTRANCE,
    DungeonRoomId.xF8_SUPER_BUNNY_CAVE_ENTRANCE_AND_CHESTS,
    DungeonRoomId.xF9_SPECTICAL_ROCK_CAVE_EXIT_AFTER_FALLING_FROM_TOP_ENTRANCE,
    DungeonRoomId.xFA_SPECTICAL_ROCK_CAVE_ENTRANCE_AFTER_JUMPING_TO_GET_TO_HP,
    DungeonRoomId.xFB_BUMPER_CAVE_ENTRANCE2,
    DungeonRoomId.xFD_SOME_CAVE_ON_DEATH_MOUNTAIN,
    DungeonRoomId.xFE_SPIRAL_CAVE_EXIT_AFTER_FALLING,
    DungeonRoomId.xFF_UPSIDE_DOWN_CAVE_MIDDLE_ENTRANCE,
    DungeonRoomId.x100_SHOP_IN_LOST_WOODS_0X100,
    DungeonRoomId.x101_SCARED_LADIES_HOUSES,
    DungeonRoomId.x102_SICK_KID,
    DungeonRoomId.x103_INN_BUSH_HOUSE,
    DungeonRoomId.x104_LINK_S_HOUSE,
    DungeonRoomId.x105_SAHASRAHLA_S_HOUSE,
    DungeonRoomId.x11A_MUTANT_HUT,
    DungeonRoomId.x106_CHEST_GAME_OUTCAST_VILLAGE_BOMB_HOUSE,
    DungeonRoomId.x107_LIBRARY_BOMB_FARM_ROOM,
    DungeonRoomId.x108_CHICKEN_HOUSE,
    DungeonRoomId.x109_WITCH_HUT,
    DungeonRoomId.x10A_AGINAH_S_CAVE,
    DungeonRoomId.x10B_SWAMP_FLOODWAY_ROOM,
    DungeonRoomId.x10C_MIMIC_CAVE,
    DungeonRoomId.x10D_CAVE_OUTSIDE_MISERY_MIRE,
    DungeonRoomId.x10E_CAVE_0X10E_2_UNKNOWN_CAVES,
    DungeonRoomId.x10F_SHOP_0X10F,
    DungeonRoomId.x110_SHOP_0X110,
    DungeonRoomId.x111_ARCHER_GAME,
    DungeonRoomId.x112_CAVE_SHOP_0X112,
    DungeonRoomId.x113_KING_S_TOMB,
    DungeonRoomId.x114_WISHING_WELL_CAVE_0X114,
    DungeonRoomId.x115_WISHING_WELL_BIG_FAIRY,
    DungeonRoomId.x116_GREATEST_FAIRY,
    DungeonRoomId.x117_SPIKE_CAVE,
    DungeonRoomId.x118_SHOP_0X118,
    DungeonRoomId.x119_BLIND_S_HOUSE,
    DungeonRoomId.x11B_MIRROR_CAVES_SOUTH_OF_TREE_BOY_ABOVE_KINGS_TOMB,
    DungeonRoomId.x11C_BOMB_SHOP,
    DungeonRoomId.x11D_BLIND_S_BASEMENT,
    DungeonRoomId.x11E_HYPE_CAVE,
    DungeonRoomId.x11F_SHOP_0X11F,
    DungeonRoomId.x120_ICE_ROD_CAVE,
    DungeonRoomId.x121_SMITHS_HOUSE,
    DungeonRoomId.x122_FORTUNE_TELLER_S,
    DungeonRoomId.x123_MINI_MOLDORM_CAVE,
    DungeonRoomId.x124_UNKNOWN_CAVE_BONK_CAVE,
    DungeonRoomId.x125_CAVE_0X125,
    DungeonRoomId.x126_CHECKER_BOARD_CAVE,
    DungeonRoomId.x127_HAMMER_PEG_CAVE,
    DungeonRoomId.x2D_EMPTY_CLONE_ROOM,
    DungeonRoomId.x05_EMPTY_CLONE_ROOM1,
    DungeonRoomId.x0F_EMPTY_CLONE_ROOM2,
    DungeonRoomId.x25_EMPTY_CLONE_ROOM3,
    DungeonRoomId.x47_EMPTY_CLONE_ROOM4,
    DungeonRoomId.x48_EMPTY_CLONE_ROOM5,
    DungeonRoomId.x69_EMPTY_CLONE_ROOM6,
    DungeonRoomId.x6F_EMPTY_CLONE_ROOM7,
    DungeonRoomId.x78_EMPTY_CLONE_ROOM8,
    DungeonRoomId.x79_EMPTY_CLONE_ROOM9,
    DungeonRoomId.x7A_EMPTY_CLONE_ROOM10,
    DungeonRoomId.x86_EMPTY_CLONE_ROOM11,
    DungeonRoomId.x88_EMPTY_CLONE_ROOM12,
    DungeonRoomId.x8A_EMPTY_CLONE_ROOM13,
    DungeonRoomId.x8F_EMPTY_CLONE_ROOM14,
    DungeonRoomId.x94_EMPTY_CLONE_ROOM15,
    DungeonRoomId.x9A_EMPTY_CLONE_ROOM16,
    DungeonRoomId.xAD_EMPTY_CLONE_ROOM17,
    DungeonRoomId.xBD_EMPTY_CLONE_ROOM18,
    DungeonRoomId.xCD_EMPTY_CLONE_ROOM19,
    DungeonRoomId.xCF_EMPTY_CLONE_ROOM20,
    DungeonRoomId.xD3_EMPTY_CLONE_ROOM21,
    DungeonRoomId.xD4_EMPTY_CLONE_ROOM22,
    DungeonRoomId.xD7_EMPTY_CLONE_ROOM23,
    DungeonRoomId.xDD_EMPTY_CLONE_ROOM24,
    DungeonRoomId.xE9_EMPTY_CLONE_ROOM25,
    DungeonRoomId.xEC_EMPTY_CLONE_ROOM26,
    DungeonRoomId.xF6_EMPTY_CLONE_ROOM27,
    DungeonRoomId.xF7_EMPTY_CLONE_ROOM28,
    DungeonRoomId.xFC_EMPTY_CLONE_ROOM29,
    DungeonRoomId.xCA_UNUSED_ROOM,
]
