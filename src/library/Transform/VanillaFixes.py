# This file contains patches inconsistencies in Vanilla that affect randomization.
from .Context import Context
from ..Model import SpriteType, SpriteVulnerability


def patch_invulnerable_sprites(context: Context) -> None:
    """This makes sure the game and randomizer are aligned on what is killable."""
    for sprite in context.sprites.values():
        meta = sprite.id.meta()
        if (
            meta.role == SpriteType.HAZARD
            or meta.vulnerability == SpriteVulnerability.INVULNERABLE
        ):
            sprite.statis = True
