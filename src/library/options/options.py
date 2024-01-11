from attr import dataclass
from library.model.balancing import Balancing
from library.options.overworld_enemy_shuffle import OverworldEnemyShuffle
from library.options.underworld_enemy_shuffle import UnderworldEnemyShuffle


@dataclass
class Options:
    seed: str
    """Provide a random generated seed. See the adjuster.py file for an example of how to generate this."""
    boss_shuffle = False
    """Experimental"""
    overworld_balancing: Balancing = Balancing.BALANCED
    """Controls the general difficulty of enemies and other settings in the overworld."""
    overworld_enemy_shuffle: OverworldEnemyShuffle = OverworldEnemyShuffle.VANILLA
    """Controls the source of enemies for the Overworld."""
    underworld_balancing: Balancing = Balancing.BALANCED
    """Controls the general difficulty of enemies and other settings in the underworld."""
    underworld_enemy_shuffle: UnderworldEnemyShuffle = UnderworldEnemyShuffle.VANILLA
    """Controls the source of enemies for the Underworld."""
    underworld_palette_shuffle = False
    """True if the underworld palettes can be shuffled."""
    underworld_tileset_shuffle = False
    """Experimental"""
    killable_thieves = False
    """Experimental"""
    mushroom_shuffle = False
    """True if the Mushroom in the Lost Woods should be shuffled with Fake Swords."""
    shadow_bees = False
    """True if bees should be mostly invisible and have more health."""
