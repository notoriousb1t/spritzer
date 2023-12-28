from .Context import Context
from ..Model import SpriteId, SpriteSubclassId


def patch_thief_killable(context: Context) -> None:
    thief = context.sprites[SpriteId.xC4_THIEF]
    thief.hp = 8
    thief.subclass = SpriteSubclassId.x1  # Same as Soldier
