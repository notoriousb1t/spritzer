from ..Model.OverworldAreaId import OverworldAreaId
from .Context import Context


from ..Model import SpriteId


def reroll_lost_woods_mushroom(context: Context) -> None:
    # Find lost woods and current mushroom.
    lost_woods_lw = context.overworld_areas[OverworldAreaId.x0_LOST_WOODS]

    original_mushroom = next(
        it for it in lost_woods_lw.light_world_v2.sprites if it.id == SpriteId.xE7_MUSHROOM
    )
    # Reassign the mushroom to a fake master sword
    original_mushroom.id = SpriteId.xE8_FAKE_MASTER_SWORD
    next_mushroom = context.random.choice(
        [it for it in lost_woods_lw.light_world_v2.sprites if it.id == SpriteId.xE8_FAKE_MASTER_SWORD]
    )
    # Randomly assign fake master sword as mushroom.
    next_mushroom.id = SpriteId.xE7_MUSHROOM
