from library.model.damage_subclass_id import DamageSubclassId
from library.model.sprite import Sprite
from library.model.sprite_id import SpriteId
from library.model.model import Model


def patch_thief_killable(context: Model) -> None:
    thief: Sprite = context.sprites[SpriteId.xC4_THIEF]
    thief.hp = 4
    thief.subclass = DamageSubclassId.x4  # Same damage profile as green eyegore
    thief.immune_to_powder = False
    thief.high_priority = True
    thief.impervious = False
    thief.harmless = False
