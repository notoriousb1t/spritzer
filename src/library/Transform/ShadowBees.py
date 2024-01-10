from .Context import Context
from ..Model import SpriteId


def patch_shadow_bees(context: Context) -> None:
    bees = context.sprites[SpriteId.x79_BEE]
    bees.display_allocation = 0
    bees.draw_shadow = True
    bees.statis = True
    bees.boss_damage_sfx = True
    bees.hp = 0x20
