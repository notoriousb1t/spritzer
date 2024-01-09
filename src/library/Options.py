from enum import StrEnum
from attr import dataclass

from library.Transform.Context import Balancing


class OverworldEnemyShuffle(StrEnum):
    VANILLA = "vanilla"
    SIMPLE = "simple"
    INVERTED = "inverted"
    FULL = "full"
    CHAOS = "chaos"
    INSANITY = "insanity"


class UnderworldEnemyShuffle(StrEnum):
    VANILLA = "vanilla"
    SIMPLE = "simple"
    FULL = "full"
    CHAOS = "chaos"
    INSANITY = "insanity"


@dataclass
class Options:
    seed: str = None
    boss_shuffle = False
    overworld_balancing = Balancing.BALANCED
    overworld_enemy_shuffle = OverworldEnemyShuffle.VANILLA
    underworld_balancing = Balancing.BALANCED
    underworld_enemy_shuffle = UnderworldEnemyShuffle.VANILLA
    underworld_palette_shuffle = False
    underworld_tileset_shuffle = False
    killable_thieves = False
    mushroom_shuffle = False
    shadow_bees = False
