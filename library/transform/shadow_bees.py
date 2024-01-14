from library.model.sprite import Sprite
from library.model.sprite_id import SpriteId
from library.model.model import Model


def patch_shadow_bees(context: Model) -> None:
    bees: Sprite = context.sprites[SpriteId.x79_BEE]
    bees.display_allocation = 0
    bees.draw_shadow = True
    bees.statis = True
    bees.boss_damage_sfx = True
    bees.hp = 0x20
