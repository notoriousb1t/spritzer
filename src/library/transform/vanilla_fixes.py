from library.model.sprite_id import SpriteId
from library.model.sprite_configuration import SpriteConfiguration
from library.model.sprite_type import SpriteType
from library.model.sprite_vulnerability import SpriteVulnerability
from library.model.model import Model


def patch_invulnerable_sprites(context: Model) -> None:
    """This makes sure the game and randomizer are aligned on what is killable."""
    for sprite in context.sprites.values():
        meta: SpriteConfiguration = sprite.id.configuration()
        if (
            meta.role == SpriteType.HAZARD
            or meta.vulnerability == SpriteVulnerability.INVULNERABLE
            or sprite.id == SpriteId.xD8_GREEN_BOMB
        ):
            sprite.statis = True
