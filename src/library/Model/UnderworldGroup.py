from attr import dataclass
from enum import Enum, auto
from typing import Dict, List
from .UnderworldRoomId import UnderworldRoomId
from .BlocksetId import BlocksetId
from .PaletteId import PaletteId


class UnderworldGroupId(Enum):
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
class UnderworldGroup:
    """A group of Underworld Rooms. This may be related cave rooms or an entire dungeon."""

    rooms: List[UnderworldRoomId] = []
    blocksets: List[BlocksetId] = []
    palettes: List[PaletteId] = []
    exclude_from_tile_shuffle: List[UnderworldRoomId] = []


def get_underworld_groups() -> Dict[UnderworldGroupId, UnderworldGroup]:
    return {
        UnderworldGroupId.HYRULE_CASTLE_ENTRANCE: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x01_HYRULE_CASTLE_NORTH_CORRIDOR,
                UnderworldRoomId.x50_HYRULE_CASTLE_WEST_CORRIDOR,
                UnderworldRoomId.x51_HYRULE_CASTLE_THRONE_ROOM,
                UnderworldRoomId.x52_HYRULE_CASTLE_EAST_CORRIDOR,
                UnderworldRoomId.x60_HYRULE_CASTLE_WEST_ENTRANCE_ROOM,
                UnderworldRoomId.x61_HYRULE_CASTLE_MAIN_ENTRANCE_ROOM,
                UnderworldRoomId.x62_HYRULE_CASTLE_EAST_ENTRANCE_ROOM,
            ],
            blocksets=[
                BlocksetId.x0_CASTLE,
                BlocksetId.x1_DUNGEON,
                BlocksetId.x2_AGAHNIM,
                BlocksetId.x5_EASTERN_PALACE,
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.x8_SWAMP_PALACE,
                BlocksetId.x9_SKULL_WOODS,
                BlocksetId.xA_THIEVES_TOWN,
                BlocksetId.xB_ICE_PALACE,
                BlocksetId.xC_MISERY_MIRE,
                BlocksetId.xD_TURTLE_ROCK,
                BlocksetId.xE_DESERT_PALACE,
                BlocksetId.x12_FAIRY_CAVE,
                BlocksetId.x13_GANON,
            ],
        ),
        UnderworldGroupId.HYRULE_CASTLE_DUNGEON: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x41_HYRULE_CASTLE_FIRST_DARK_ROOM,
                UnderworldRoomId.x55_CASTLE_SECRET_ENTRANCE_UNCLE_DEATH_ROOM,
                UnderworldRoomId.x70_HYRULE_CASTLE_SMALL_CORRIDOR_TO_JAIL_CELLS,
                UnderworldRoomId.x71_HYRULE_CASTLE_BOOMERANG_CHEST_ROOM,
                UnderworldRoomId.x72_HYRULE_CASTLE_MAP_CHEST_ROOM,
                UnderworldRoomId.x80_HYRULE_CASTLE_JAIL_CELL_ROOM,
                UnderworldRoomId.x81_HYRULE_CASTLE_NEXT_TO_CHASM_ROOM,
                UnderworldRoomId.x82_HYRULE_CASTLE_BASEMENT_CHASM_ROOM,
            ],
            blocksets=[
                BlocksetId.x0_CASTLE,
                BlocksetId.x1_DUNGEON,
                BlocksetId.x2_AGAHNIM,
                BlocksetId.x5_EASTERN_PALACE,
                BlocksetId.x6_TOWER1,
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.x8_SWAMP_PALACE,
                BlocksetId.x9_SKULL_WOODS,
                BlocksetId.xA_THIEVES_TOWN,
                BlocksetId.xB_ICE_PALACE,
                BlocksetId.xC_MISERY_MIRE,
                BlocksetId.xD_TURTLE_ROCK,
                BlocksetId.xE_DESERT_PALACE,
                BlocksetId.x12_FAIRY_CAVE,
                BlocksetId.x13_GANON,
                BlocksetId.x14_CAVE,
            ],
        ),
        UnderworldGroupId.HYRULE_CASTLE_ESCAPE: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x11_HYRULE_CASTLE_BOMBABLE_STOCK_ROOM,
                UnderworldRoomId.x21_HYRULE_CASTLE_KEY_RAT_ROOM,
                UnderworldRoomId.x22_HYRULE_CASTLE_SEWER_TEXT_TRIGGER_ROOM,
                UnderworldRoomId.x32_HYRULE_CASTLE_SEWER_KEY_CHEST_ROOM,
                UnderworldRoomId.x42_HYRULE_CASTLE_6_ROPES_ROOM,
            ],
            blocksets=[
                BlocksetId.x0_CASTLE,
                BlocksetId.x1_DUNGEON,
                BlocksetId.x2_AGAHNIM,
                BlocksetId.x5_EASTERN_PALACE,
                BlocksetId.x6_TOWER1,
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.x8_SWAMP_PALACE,
                BlocksetId.x9_SKULL_WOODS,
                BlocksetId.xA_THIEVES_TOWN,
                BlocksetId.xB_ICE_PALACE,
                BlocksetId.xC_MISERY_MIRE,
                BlocksetId.xD_TURTLE_ROCK,
                BlocksetId.xE_DESERT_PALACE,
                BlocksetId.x12_FAIRY_CAVE,
                BlocksetId.x13_GANON,
                BlocksetId.x14_CAVE,
            ],
        ),
        UnderworldGroupId.SANCTUARY: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x02_HYRULE_CASTLE_SWITCH_ROOM,
                UnderworldRoomId.x12_SANCTUARY,
            ],
            blocksets=[
                BlocksetId.x4_SANTUARY,
            ],
        ),
        UnderworldGroupId.DESERT_PALACE_FRONT: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x73_DESERT_PALACE_BIG_CHEST_ROOM,
                UnderworldRoomId.x74_DESERT_PALACE_MAP_CHEST_ROOM,
                UnderworldRoomId.x75_DESERT_PALACE_BIG_KEY_CHEST_ROOM,
                UnderworldRoomId.x83_DESERT_PALACE_WEST_ENTRANCE_ROOM,
                UnderworldRoomId.x84_DESERT_PALACE_MAIN_ENTRANCE_ROOM,
                UnderworldRoomId.x85_DESERT_PALACE_EAST_ENTRANCE_ROOM,
            ],
            blocksets=[
                BlocksetId.x5_EASTERN_PALACE,
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.xA_THIEVES_TOWN,
                BlocksetId.xE_DESERT_PALACE,
                BlocksetId.x12_FAIRY_CAVE,
            ],
        ),
        UnderworldGroupId.DESERT_PALACE_FRONT: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x33_DESERT_PALACE_LANMOLAS_BOSS,
                UnderworldRoomId.x43_DESERT_PALACE_TORCH_PUZZLE_MOVING_WALL_ROOM,
                UnderworldRoomId.x53_DESERT_PALACE_POPOS_2_BEAMOS_HELLWAY_ROOM,
                UnderworldRoomId.x63_DESERT_PALACE_FINAL_SECTION_ENTRANCE_ROOM,
            ],
            blocksets=[
                BlocksetId.x1_DUNGEON,
                BlocksetId.x5_EASTERN_PALACE,
                BlocksetId.x6_TOWER1,
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.x8_SWAMP_PALACE,
                BlocksetId.x9_SKULL_WOODS,
                BlocksetId.xB_ICE_PALACE,
                BlocksetId.xC_MISERY_MIRE,
                BlocksetId.xE_DESERT_PALACE,
                BlocksetId.x12_FAIRY_CAVE,
                BlocksetId.x13_GANON,
            ],
        ),
        UnderworldGroupId.EASTERN_PALACE: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x89_EASTERN_PALACE_FAIRY_ROOM,
                UnderworldRoomId.x99_EASTERN_PALACE_EYEGORE_KEY_ROOM,
                UnderworldRoomId.xA8_EASTERN_PALACE_STALFOS_SPAWN_ROOM,
                UnderworldRoomId.xA9_EASTERN_PALACE_BIG_CHEST_ROOM,
                UnderworldRoomId.xAA_EASTERN_PALACE_MAP_CHEST_ROOM,
                UnderworldRoomId.xB8_EASTERN_PALACE_BIG_KEY_ROOM,
                UnderworldRoomId.xB9_EASTERN_PALACE_LOBBY_CANNONBALLS_ROOM,
                UnderworldRoomId.xBA_EASTERN_PALACE_DARK_ANTIFAIRY_KEY_POT_ROOM,
                UnderworldRoomId.xC8_EASTERN_PALACE_ARMOS_KNIGHTS_BOSS,
                UnderworldRoomId.xC9_EASTERN_PALACE_ENTRANCE_ROOM,
                UnderworldRoomId.xD8_EASTERN_PALACE_ZELDAGAMER_ROOM_PRE_ARMOS_KNIGHTS_ROOM,
                UnderworldRoomId.xD9_EASTERN_PALACE_CANONBALL_ROOM,
                UnderworldRoomId.xDA_EASTERN_PALACE_2_BUBBLE_WITH_SWITCH_UNDER_POT,
            ],
            exclude_from_tile_shuffle=[
                UnderworldRoomId.xA9_EASTERN_PALACE_BIG_CHEST_ROOM,
                UnderworldRoomId.xD9_EASTERN_PALACE_CANONBALL_ROOM,
                UnderworldRoomId.xB9_EASTERN_PALACE_LOBBY_CANNONBALLS_ROOM,
            ],
            blocksets=[
                BlocksetId.x0_CASTLE,
                BlocksetId.x1_DUNGEON,
                BlocksetId.x2_AGAHNIM,
                BlocksetId.x4_SANTUARY,
                BlocksetId.x5_EASTERN_PALACE,
                BlocksetId.x6_TOWER1,
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.x8_SWAMP_PALACE,
                BlocksetId.x9_SKULL_WOODS,
                BlocksetId.xA_THIEVES_TOWN,
                BlocksetId.xB_ICE_PALACE,
                BlocksetId.xC_MISERY_MIRE,
                BlocksetId.xD_TURTLE_ROCK,
                BlocksetId.xE_DESERT_PALACE,
                BlocksetId.xF_SAHASRAHLA,
                BlocksetId.x14_CAVE,
            ],
        ),
        UnderworldGroupId.TOWER_OF_HERA: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x07_TOWER_OF_HERA_MOLDORM_BOSS,
                UnderworldRoomId.x17_TOWER_OF_HERA_MOLDORM_FALL_ROOM,
                UnderworldRoomId.x27_TOWER_OF_HERA_BIG_CHEST,
                UnderworldRoomId.x31_TOWER_OF_HERA_HARDHAT_BEETLES_ROOM,
                UnderworldRoomId.x77_TOWER_OF_HERA_ENTRANCE_ROOM,
                UnderworldRoomId.x87_TOWER_OF_HERA_TILE_ROOM,
                UnderworldRoomId.xA7_TOWER_OF_HERA_FAIRY_ROOM,
            ],
            blocksets=[
                BlocksetId.x0_CASTLE,
                BlocksetId.x1_DUNGEON,
                BlocksetId.x2_AGAHNIM,
                BlocksetId.x5_EASTERN_PALACE,
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.x8_SWAMP_PALACE,
                BlocksetId.x9_SKULL_WOODS,
                BlocksetId.xA_THIEVES_TOWN,
                BlocksetId.xB_ICE_PALACE,
                BlocksetId.xC_MISERY_MIRE,
                BlocksetId.xE_DESERT_PALACE,
                BlocksetId.x12_FAIRY_CAVE,
                BlocksetId.x13_GANON,
            ],
        ),
        UnderworldGroupId.AGAHNIMS_TOWER: UnderworldGroup(
            rooms=[
                # No compatible sets for these two
                # UnderworldRoomId.x20_AGAHNIM_S_TOWER_AGAHNIM_BOSS,
                # UnderworldRoomId.x30_AGAHNIM_S_TOWER_MAIDEN_SACRIFICE_CHAMBER,
                UnderworldRoomId.x40_AGAHNIM_S_TOWER_FINAL_BRIDGE_ROOM,
                UnderworldRoomId.xB0_AGAHNIM_S_TOWER_CIRCLE_OF_POTS,
                UnderworldRoomId.xC0_AGAHNIM_S_TOWER_DARK_BRIDGE_ROOM,
                UnderworldRoomId.xD0_AGAHNIM_S_TOWER_DARK_MAZE,
                UnderworldRoomId.xE0_AGAHNIM_S_TOWER_ENTRANCE_ROOM,
            ],
            blocksets=[
                BlocksetId.x2_AGAHNIM,
                BlocksetId.x9_SKULL_WOODS,
            ],
        ),
        UnderworldGroupId.PALACE_OF_DARKNESS: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x09_PALACE_OF_DARKNESS,
                UnderworldRoomId.x0A_PALACE_OF_DARKNESS_STALFOS_TRAP_ROOM,
                UnderworldRoomId.x0B_PALACE_OF_DARKNESS_TURTLE_ROOM,
                UnderworldRoomId.x19_PALACE_OF_DARKNESS_DARK_MAZE,
                UnderworldRoomId.x1A_PALACE_OF_DARKNESS_BIG_CHEST_ROOM,
                UnderworldRoomId.x1B_PALACE_OF_DARKNESS_MIMICS_MOVING_WALL_ROOM,
                # Tileset must be x7 otherwise the hop tiles break.
                # UnderworldRoomId.x2A_PALACE_OF_DARKNESS_BIG_HUB_ROOM,
                UnderworldRoomId.x2B_PALACE_OF_DARKNESS_MAP_CHEST_FAIRY_ROOM,
                UnderworldRoomId.x3A_PALACE_OF_DARKNESS_BOMBABLE_FLOOR_ROOM,
                UnderworldRoomId.x3B_PALACE_OF_DARKNESS_SPIKE_BLOCK_CONVEYOR_ROOM,
                UnderworldRoomId.x4A_PALACE_OF_DARKNESS_ENTRANCE_ROOM,
                UnderworldRoomId.x4B_PALACE_OF_DARKNESS_WARPS_SOUTH_MIMICS_ROOM,
                UnderworldRoomId.x5A_PALACE_OF_DARKNESS_HELMASAUR_KING_BOSS,
                UnderworldRoomId.x6A_PALACE_OF_DARKNESS_RUPEE_ROOM,
            ],
            blocksets=[
                BlocksetId.x5_EASTERN_PALACE,
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.xB_ICE_PALACE,
                BlocksetId.xC_MISERY_MIRE,
                BlocksetId.x12_FAIRY_CAVE,
            ],
        ),
        UnderworldGroupId.SWAMP_PALACE: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x06_SWAMP_PALACE_ARRGHUS_BOSS,
                UnderworldRoomId.x16_SWAMP_PALACE_SWIMMING_TREADMILL,
                UnderworldRoomId.x26_SWAMP_PALACE_STATUE_ROOM,
                UnderworldRoomId.x28_SWAMP_PALACE_ENTRANCE_ROOM,
                UnderworldRoomId.x34_SWAMP_PALACE_PUSH_BLOCK_PUZZLE_PRE_BIG_KEY_ROOM,
                UnderworldRoomId.x35_SWAMP_PALACE_BIG_KEY_BS_ROOM,
                UnderworldRoomId.x36_SWAMP_PALACE_BIG_CHEST_ROOM,
                UnderworldRoomId.x37_SWAMP_PALACE_MAP_CHEST_WATER_FILL_ROOM,
                UnderworldRoomId.x38_SWAMP_PALACE_KEY_POT_ROOM,
                UnderworldRoomId.x46_SWAMP_PALACE_COMPASS_CHEST_ROOM,
                UnderworldRoomId.x54_SWAMP_PALACE_UPSTAIRS_PITS_ROOM,
                UnderworldRoomId.x66_SWAMP_PALACE_HIDDEN_CHEST_HIDDEN_DOOR_ROOM,
                UnderworldRoomId.x76_SWAMP_PALACE_WATER_DRAIN_ROOM,
            ],
            blocksets=[
                # Nothing else works for this without removing decorative vents.
                BlocksetId.x8_SWAMP_PALACE
            ],
        ),
        UnderworldGroupId.SKULL_WOODS: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x29_SKULL_WOODS_MOTHULA_BOSS,
                UnderworldRoomId.x39_SKULL_WOODS_GIBDO_KEY_MOTHULA_HOLE_ROOM,
                UnderworldRoomId.x49_SKULL_WOODS_GIBDO_TORCH_PUZZLE_ROOM,
                UnderworldRoomId.x56_SKULL_WOODS_KEY_POT_TRAP_ROOM,
                UnderworldRoomId.x57_SKULL_WOODS_BIG_KEY_ROOM,
                UnderworldRoomId.x58_SKULL_WOODS_BIG_CHEST_ROOM,
                UnderworldRoomId.x59_SKULL_WOODS_FINAL_SECTION_ENTRANCE_ROOM,
                UnderworldRoomId.x67_SKULL_WOODS_COMPASS_CHEST_ROOM,
                UnderworldRoomId.x68_SKULL_WOODS_KEY_CHEST_TRAP_ROOM,
            ],
            blocksets=[
                BlocksetId.x0_CASTLE,
                BlocksetId.x2_AGAHNIM,
                BlocksetId.x9_SKULL_WOODS,
            ],
        ),
        UnderworldGroupId.ICE_PALACE: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x0E_ICE_PALACE_ENTRANCE_ROOM,
                UnderworldRoomId.x1E_ICE_PALACE_BOMB_FLOOR_BARI_ROOM,
                UnderworldRoomId.x1F_ICE_PALACE_PENGATOR_BIG_KEY_ROOM,
                UnderworldRoomId.x2E_ICE_PALACE_COMPASS_ROOM,
                UnderworldRoomId.x3E_ICE_PALACE_STALFOS_KNIGHTS_CONVEYOR_HELLWAY,
                UnderworldRoomId.x3F_ICE_PALACE_MAP_CHEST_ROOM,
                UnderworldRoomId.x4E_ICE_PALACE_BOMB_JUMP_ROOM,
                UnderworldRoomId.x4F_ICE_PALACE_CLONE_ROOM_FAIRY_ROOM,
                UnderworldRoomId.x5E_ICE_PALACE_LONELY_FIREBAR,
                UnderworldRoomId.x5F_ICE_PALACE_HIDDEN_CHEST_SPIKE_FLOOR_ROOM,
                UnderworldRoomId.x6E_ICE_PALACE_PENGATORS_ROOM,
                UnderworldRoomId.x7E_ICE_PALACE_HIDDEN_CHEST_BOMBABLE_FLOOR_ROOM,
                UnderworldRoomId.x7F_ICE_PALACE_BIG_SPIKE_TRAPS_ROOM,
                UnderworldRoomId.x8E_ICE_PALACE_BLOBS_WITH_TETRIS_BARRIER,
                UnderworldRoomId.x9E_ICE_PALACE_BIG_CHEST_ROOM,
                UnderworldRoomId.x9F_ICE_PALACE_ROOM_WITH_ICE_FLOOR_KEY_AND_4_WALL_RATS,
                UnderworldRoomId.xAE_ICE_PALACE_2_BLUE_BARI_AND_HIDDEN_CHEST,
                UnderworldRoomId.xAF_ICE_PALACE_ICE_BRIDGE_ROOM,
                UnderworldRoomId.xBE_ICE_PALACE_BLOCK_PUZZLE_ROOM,
                UnderworldRoomId.xBF_ICE_PALACE_CLONE_ROOM_SWITCH_ROOM,
                UnderworldRoomId.xCE_ICE_PALACE_HOLE_TO_KHOLDSTARE_ROOM,
                UnderworldRoomId.xDE_ICE_PALACE_KHOLDSTARE_BOSS,
            ],
            blocksets=[
                # Freezors restrict this to only this tileset unless we patch tiles.
                BlocksetId.xB_ICE_PALACE
            ],
        ),
        UnderworldGroupId.MISERY_MIRE: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x90_MISERY_MIRE_VITREOUS_BOSS,
                UnderworldRoomId.x91_MISERY_MIRE_FINAL_SWITCH_ROOM,
                UnderworldRoomId.x92_MISERY_MIRE_DARK_BOMB_WALL_SWITCHES_ROOM,
                UnderworldRoomId.x93_MISERY_MIRE_DARK_CANE_FLOOR_SWITCH_PUZZLE_ROOM,
                UnderworldRoomId.x97_MISERY_MIRE_TORCH_PUZZLE_MOVING_WALL_ROOM,
                UnderworldRoomId.x98_MISERY_MIRE_ENTRANCE_ROOM,
                UnderworldRoomId.xA0_MISERY_MIRE_PRE_VITREOUS_ROOM,
                UnderworldRoomId.xA1_MISERY_MIRE_FISH_ROOM,
                UnderworldRoomId.xA2_MISERY_MIRE_BRIDGE_KEY_CHEST_ROOM,
                UnderworldRoomId.xA3_MISERY_MIRE_EMPTY_L_CONNECTING_ROOM,
                UnderworldRoomId.xB1_MISERY_MIRE_HOURGLASS_ROOM,
                UnderworldRoomId.xB2_MISERY_MIRE_SLUG_ROOM,
                UnderworldRoomId.xB3_MISERY_MIRE_SPIKE_KEY_CHEST_ROOM,
                UnderworldRoomId.xC1_MISERY_MIRE_COMPASS_CHEST_TILE_ROOM,
                UnderworldRoomId.xC2_MISERY_MIRE_BIG_HUB_ROOM,
                UnderworldRoomId.xC3_MISERY_MIRE_BIG_CHEST_ROOM,
                UnderworldRoomId.xD1_MISERY_MIRE_CONVEYOR_SLUG_BIG_KEY_ROOM,
                UnderworldRoomId.xD2_MISERY_MIRE_MIRE02_WIZZROBES_ROOM,
            ],
            exclude_from_tile_shuffle=[
                UnderworldRoomId.x90_MISERY_MIRE_VITREOUS_BOSS,
                UnderworldRoomId.xA0_MISERY_MIRE_PRE_VITREOUS_ROOM,
            ],
            blocksets=[
                BlocksetId.x5_EASTERN_PALACE,
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.xB_ICE_PALACE,
                BlocksetId.xC_MISERY_MIRE,
                BlocksetId.x12_FAIRY_CAVE,
            ],
        ),
        UnderworldGroupId.THIEVES_TOWN_ENTRANCE: UnderworldGroup(
            rooms=[
                # The light effect makes this not possible to switch.
                # UnderworldRoomId.xAC_THIEVES_TOWN_BLIND_THE_THIEF_BOSS,
                UnderworldRoomId.xCB_THIEVES_TOWN_NORTH_WEST_ENTRANCE_ROOM,
                UnderworldRoomId.xCC_THIEVES_TOWN_NORTH_EAST_ENTRANCE_ROOM,
                UnderworldRoomId.xDB_THIEVES_TOWN_MAIN_SOUTH_WEST_ENTRANCE_ROOM,
                UnderworldRoomId.xDC_THIEVES_TOWN_SOUTH_EAST_ENTRANCE_ROOM,
                UnderworldRoomId.xBB_THIEVES_TOWN_HELLWAY,
                UnderworldRoomId.xBC_THIEVES_TOWN_CONVEYOR_TOILET,
                UnderworldRoomId.xAB_THIEVES_TOWN_MOVING_SPIKES_KEY_POT_ROOM,
            ],
            blocksets=[
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.x9_SKULL_WOODS,
                BlocksetId.xA_THIEVES_TOWN,
                BlocksetId.xB_ICE_PALACE,
                BlocksetId.xC_MISERY_MIRE,
                BlocksetId.xE_DESERT_PALACE,
            ],
        ),
        UnderworldGroupId.THIEVES_TOWN_ATTIC: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x64_THIEVES_TOWN_WEST_ATTIC_ROOM,
                UnderworldRoomId.x65_THIEVES_TOWN_EAST_ATTIC_ROOM,
            ],
            blocksets=[
                # Shafts of light make a switch not possible
                BlocksetId.xA_THIEVES_TOWN,
            ],
        ),
        UnderworldGroupId.THIEVES_TOWN_BASEMENT: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x44_THIEVES_TOWN_BIG_CHEST_ROOM,
                UnderworldRoomId.x45_THIEVES_TOWN_JAIL_CELLS_ROOM,
            ],
            blocksets=[
                BlocksetId.x7_TOWER_HERA,
                BlocksetId.x9_SKULL_WOODS,
                BlocksetId.xA_THIEVES_TOWN,
                BlocksetId.xB_ICE_PALACE,
                BlocksetId.xC_MISERY_MIRE,
                BlocksetId.xE_DESERT_PALACE,
            ],
        ),
        UnderworldGroupId.TURTLE_ROCK: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x04_TURTLE_ROCK_CRYSTA_ROLLER_ROOM,
                UnderworldRoomId.x13_TURTLE_ROCK_HOKKU_BOKKU_KEY_ROOM_2,
                UnderworldRoomId.x14_TURTLE_ROCK_BIG_KEY_ROOM,
                UnderworldRoomId.x15_TURTLE_ROCK,
                UnderworldRoomId.x23_TURTLE_ROCK_WEST_EXIT_TO_BALCONY,
                UnderworldRoomId.x24_TURTLE_ROCK_DOUBLE_HOKKU_BOKKU_BIG_CHEST_ROOM,
                UnderworldRoomId.xA4_TURTLE_ROCK_TRINEXX_BOSS,
                UnderworldRoomId.xB4_TURTLE_ROCK_PRE_TRINEXX_ROOM,
                UnderworldRoomId.xB5_TURTLE_ROCK_DARK_MAZE,
                UnderworldRoomId.xB6_TURTLE_ROCK_CHAIN_CHOMPS_ROOM,
                UnderworldRoomId.xB7_TURTLE_ROCK_MAP_CHEST_KEY_CHEST_ROLLER_ROOM,
                UnderworldRoomId.xC4_TURTLE_ROCK_FINAL_CRYSTAL_SWITCH_PUZZLE_ROOM,
                UnderworldRoomId.xC5_TURTLE_ROCK_LASER_BRIDGE,
                UnderworldRoomId.xC6_TURTLE_ROCK_AFTER_ENTRANCE_SQUARE_SOMARIA_PLATFORM_ROOM,
                UnderworldRoomId.xC7_TURTLE_ROCK_TORCH_PUZZLE,
                UnderworldRoomId.xD5_TURTLE_ROCK_LASER_KEY_ROOM,
                UnderworldRoomId.xD6_TURTLE_ROCK_ENTRANCE_ROOM,
            ],
            blocksets=[],
        ),
        UnderworldGroupId.GANONS_TOWER: UnderworldGroup(
            rooms=[
                UnderworldRoomId.x0C_GANON_S_TOWER_ENTRANCE_ROOM,
                UnderworldRoomId.x0D_GANON_S_TOWER_AGAHNIM2_BOSS,
                UnderworldRoomId.x1C_GANON_S_TOWER_ICE_ARMOS,
                UnderworldRoomId.x1D_GANON_S_TOWER_FINAL_HALLWAY,
                UnderworldRoomId.x3D_GANON_S_TOWER_TORCH_ROOM_2,
                UnderworldRoomId.x4C_GANON_S_TOWER_MINI_HELMASAUR_CONVEYOR_ROOM,
                UnderworldRoomId.x4D_GANON_S_TOWER_MOLDORM_ROOM,
                UnderworldRoomId.x5B_GANON_S_TOWER_SPIKE_PIT_ROOM,
                UnderworldRoomId.x5C_GANON_S_TOWER_GANON_BALL_Z,
                UnderworldRoomId.x5D_GANON_S_TOWER_GAUNTLET_1_2_3,
                UnderworldRoomId.x6B_GANON_S_TOWER_MIMICS_ROOMS,
                UnderworldRoomId.x6C_GANON_S_TOWER_LANMOLAS_ROOM,
                UnderworldRoomId.x6D_GANON_S_TOWER_GAUNTLET_4_5,
                UnderworldRoomId.x7B_GANON_S_TOWER_SIDWAYS_CONVEYORS_4_CHEST_4_SHOOTER_SQUARE_PIT,
                UnderworldRoomId.x7C_GANON_S_TOWER_EAST_SIDE_COLLAPSING_BRIDGE_EXPLODING_WALL_ROOM,
                UnderworldRoomId.x7D_GANON_S_TOWER_WINDER_WARP_MAZE_ROOM,
                UnderworldRoomId.x8B_GANON_S_TOWER_BLOCK_PUZZLE_SPIKE_SKIP_MAP_CHEST_ROOM,
                UnderworldRoomId.x8C_GANON_S_TOWER_EAST_AND_WEST_DOWNSTAIRS_BIG_CHEST_ROOM,
                UnderworldRoomId.x8D_GANON_S_TOWER_TILE_TORCH_PUZZLE_ROOM,
                UnderworldRoomId.x95_GANON_S_TOWER_FINAL_COLLAPSING_BRIDGE_ROOM,
                UnderworldRoomId.x96_GANON_S_TOWER_TORCHES_1_ROOM,
                UnderworldRoomId.x9B_GANON_S_TOWER_MANY_SPIKES_WARP_MAZE_ROOM,
                UnderworldRoomId.x9C_GANON_S_TOWER_INVISIBLE_FLOOR_MAZE_ROOM,
                UnderworldRoomId.x9D_GANON_S_TOWER_COMPASS_CHEST_INVISIBLE_FLOOR_ROOM,
                UnderworldRoomId.xA5_GANON_S_TOWER_WIZZROBES_ROOMS,
                UnderworldRoomId.xA6_GANON_S_TOWER_MOLDORM_FALL_ROOM,
            ],
            blocksets=[],
        ),
    }
