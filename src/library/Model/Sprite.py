from attr import dataclass

from .SpriteFlags0 import SpriteFlags0
from .SpriteFlags3 import SpriteFlags3
from .SpriteId import SpriteId
from .SpriteSubclassId import SpriteSubclassId

@dataclass
class Sprite:
    id: SpriteId
    """Entity Id."""
    hp: int = None
    """Amount of health it has."""
    ignore_recoil_collision: bool = None
    """True if collision should be ignored while Link is recoiling."""
    high_priority: bool = None
    """Mostly determines what bees should target."""
    immune_to_powder: bool = None
    """True if the creature is immune to Magic Powder."""
    boss_prep_preserved: bool = None
    """True if the creature is not deleted as part of boss prep."""
    subclass: SpriteSubclassId = None
    """Damage profile (determines damage table)."""
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
