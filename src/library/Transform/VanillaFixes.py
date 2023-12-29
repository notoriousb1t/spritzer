# This file contains patches inconsistencies in Vanilla that affect randomization.
from .Context import Context
from ..Model import SpriteId, SpriteType, SpriteVulnerability


def patch_invulnerable_sprites(context: Context) -> None:
    for sprite_id in list(SpriteId):
        if (
            sprite_id.meta().role == SpriteType.HAZARD
            or sprite_id.meta().vulnerability == SpriteVulnerability.INVULNERABLE
        ):
            context.sprites[sprite_id].statis = True
