from attr import dataclass
from typing import Union

from library.model.spriteset_id import SpritesetId
from library.model.sprite_sheet_id import SpriteSheetId


@dataclass
class Spriteset:
    """Loads the a blockset of sprites. This contains up to 4 sprite sets that load in individual sprites"""

    id: SpritesetId
    sheet0: SpriteSheetId = SpriteSheetId.x0_FREESPACE
    sheet1: SpriteSheetId = SpriteSheetId.x0_FREESPACE
    sheet2: SpriteSheetId = SpriteSheetId.x0_FREESPACE
    sheet3: SpriteSheetId = SpriteSheetId.x0_FREESPACE

    def get_at(self, index: int) -> Union[SpriteSheetId, None]:
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
