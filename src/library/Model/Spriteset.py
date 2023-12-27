from attr import dataclass

from .SpritesetId import SpritesetId



@dataclass
class Spriteset:
    """Loads the a blockset of sprites. This contains up to 4 sprite sets that load in individual sprites"""

    id: SpritesetId
    sheet0: int = None
    sheet1: int = None
    sheet2: int = None
    sheet3: int = None
