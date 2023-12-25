from attr import dataclass

from .SpriteFlags0 import SpriteFlags0
from .SpriteFlags3 import SpriteFlags3
from .SpriteId import SpriteId


@dataclass
class Sprite:
    id: SpriteId
    """Entity Id."""
    hp: int = None
    """Amount of health it has."""
    damage: int = None
    """Damage caused to Link."""
    flags_0: int = None
    flags_3: int = None
    settings_4: int = None
    settings_5: int = None

    def set_invisible(self) -> None:
        self.flags_0 &= ~(
            SpriteFlags0.DISPLAY_0
            | SpriteFlags0.DISPLAY_1
            | SpriteFlags0.DISPLAY_2
            | SpriteFlags0.DISPLAY_3
        )

    def set_invincible(self) -> None:
        self.flags_3 |= SpriteFlags3.INVINCIBLE
