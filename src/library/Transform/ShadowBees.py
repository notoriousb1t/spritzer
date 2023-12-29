from .Context import Context
from ..Model import SpriteId, SpriteType


def patch_shadow_bees(context: Context) -> None:
    bees = context.sprites[SpriteId.x79_BEE]
    bees.preserved_offscreen = True
    bees.impervious = True

    for sprites in [it.overworld_sprites for it in context.overworld_areas.values()]:
        for sprite in sprites:
            if sprite.id.meta().role == SpriteType.CONSUMABLE:
                sprite.id = SpriteId.x79_BEE
