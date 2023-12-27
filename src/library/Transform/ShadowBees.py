from .Context import Context
from ..Model import SpriteId


def patch_shadow_bees(context: Context) -> None:
    bees = context.sprites[SpriteId.x79_BEE]
    bees.set_invisible()
    bees.set_invincible()
