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

    def get_at(self, index: int) -> SpriteSheetId:
        if index == 0:
            return self.sheet0
        if index == 1:
            return self.sheet1
        if index == 2:
            return self.sheet2
        if index == 3:
            return self.sheet3
        return None

    def set_at(self, index: int, value: SpriteSheetId) -> None:
        if index == 0:
            self.sheet0 = value
        elif index == 1:
            self.sheet1 = value
        elif index == 2:
            self.sheet2 = value
        elif index == 3:
            self.sheet3 = value
