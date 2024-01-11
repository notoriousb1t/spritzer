from random import Random
from library.model.overworld_area import OverworldArea
from library.model.overworld_area import OverworldSprite
from library.model.overworld_area_id import OverworldAreaId
from library.model.sprite_id import SpriteId
from library.model.model import Model


def reroll_lost_woods_mushroom(context: Model) -> None:
    random = Random()
    random.seed(a=context.seed)

    # Find lost woods and current mushroom.
    lost_woods_lw: OverworldArea = context.overworld_areas[
        OverworldAreaId.x0_LOST_WOODS
    ]

    original_mushroom: OverworldSprite = next(
        it
        for it in lost_woods_lw.lw_v2.sprites
        if it.sprite_id == SpriteId.xE7_MUSHROOM
    )
    # Reassign the mushroom to a fake master sword
    original_mushroom.sprite_id = SpriteId.xE8_FAKE_MASTER_SWORD
    next_mushroom: OverworldSprite = random.choice(
        seq=[
            it
            for it in lost_woods_lw.lw_v2.sprites
            if it.sprite_id == SpriteId.xE8_FAKE_MASTER_SWORD
        ]
    )
    # Randomly assign fake master sword as mushroom.
    next_mushroom.sprite_id = SpriteId.xE7_MUSHROOM
