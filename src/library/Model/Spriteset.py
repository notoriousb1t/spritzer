from attr import dataclass

from .SpritesetId import SpritesetId
from .SpriteSheetId import SpriteSheetId



@dataclass
class Spriteset:
    """Loads the a blockset of sprites. This contains up to 4 sprite sets that load in individual sprites"""

    id: SpritesetId
    sheet0: SpriteSheetId = None
    sheet1: SpriteSheetId = None
    sheet2: SpriteSheetId = None
    sheet3: SpriteSheetId = None
