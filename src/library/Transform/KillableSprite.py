from .Context import Context
from ..Model import SpriteId, SpriteSubclassId


def patch_thief_killable(context: Context) -> None:
    thief = context.sprites[SpriteId.xC4_THIEF]
    thief.hp = 4
    thief.subclass = SpriteSubclassId.x4  # Same damage profile as green eyegore
    thief.immune_to_powder = False
    thief.high_priority = True
    thief.impervious = False
    thief.harmless = False
