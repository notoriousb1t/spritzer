from .Context import Context
from ..Model import SpriteId


def patch_thief_killable(context: Context) -> None:
    thief = context.sprites[SpriteId.xC4_THIEF]
    thief.hp = 4  # Almost no health.
    # TODO: modify weapon damage table so they can actually get hit
