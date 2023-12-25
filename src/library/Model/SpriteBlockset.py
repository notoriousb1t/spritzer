from attr import dataclass

from .SpriteBlocksetId import SpriteBlocksetId



@dataclass
class SpriteBlockset:
    """Loads the a blockset of sprites. This contains up to 4 sprite sets that load in individual sprites"""

    id: SpriteBlocksetId
    set0: int = None
    set1: int = None
    set2: int = None
    set3: int = None
