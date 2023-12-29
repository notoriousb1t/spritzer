from .Context import Context
from ..Model import SpriteId, SpriteSubclassId


def patch_thief_killable(context: Context) -> None:
    thief = context.sprites[SpriteId.xC4_THIEF]
    thief.hp = 4
    thief.subclass = SpriteSubclassId.x1  # Same damage profile as Soldier
    thief.immune_to_powder = False
    thief.high_priority = True
    thief.impervious = False
    thief.harmless = False
    thief.display_allocation = 0b11111
    debug(context)
    
def debug(context: Context):
    sprite = context.sprites[SpriteId.x42_GREEN_SWORD_SOLDIER]
