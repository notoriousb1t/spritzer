from enum import StrEnum
from attr import dataclass


class DungeonEnemyShuffle(StrEnum):
    VANILLA = "vanilla"
    SIMPLE = "simple"
    FULL = "full"
    CHAOS = "chaos"
    INSANITY = "insanity"


class OverworldEnemyShuffle(StrEnum):
    VANILLA = "vanilla"
    SIMPLE = "simple"
    INVERTED = "inverted"
    FULL = "full"
    CHAOS = "chaos"
    INSANITY = "insanity"


@dataclass
class Options:
    boss_shuffle = False
    dungeon_enemy_shuffle = DungeonEnemyShuffle.VANILLA
    dungeon_palette_shuffle = False
    dungeon_tileset_shuffle = False
    killable_thieves = False
    mushroom_shuffle = False
    overworld_enemy_shuffle = OverworldEnemyShuffle.VANILLA
    seed: str = None
    shadow_bees = False
