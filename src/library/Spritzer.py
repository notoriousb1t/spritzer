from random import Random
from typing import Callable

from .Model.Sprites import SpriteId
from .Rom import (
    load_rooms,
    save_rooms,
    write_overworld_areas,
    read_overworld_areas,
    read_sprite_blocksets,
    write_sprite_blocksets,
    read_sprites,
    write_sprites,
)
from .Transform.Context import Context
from .Transform.DungeonShuffle import reroll_dungeons
from .Transform.OverworldShuffle import reroll_overworld


class Spritzer:
    context: Context

    def __init__(self, random: Random) -> None:
        self.context = Context(random=random)

    def load(self, read: Callable[[int], int]) -> None:
        self.context.assert_unloaded()

        self.context.sprites = read_sprites(read)
        self.context.overworld_areas = write_overworld_areas(read)
        self.context.dungeon_rooms = load_rooms(read)
        self.context.sprite_blocksets = read_sprite_blocksets(read)
        self.context.loaded = True

        areas = list(self.context.overworld_areas.values())
        areas.sort(key=lambda it: it.id)

        # for area in areas:
        #     print(
        #         f" {hex(area.sprite_blockset_address)} | {hex(area.id.value)} | {area.id} | {area.blockset_id}"
        #     )

    def enable_sprite_shuffle_simple(self) -> None:
        self.context.assert_loaded()

        reroll_overworld(self.context)
        reroll_dungeons(self.context)

    def enable_sprite_shuffle_dungeonssimple(self) -> None:
        self.context.assert_loaded()

        reroll_dungeons(self.context)

    def enable_killable_thieves(self) -> None:
        thief = self.context.sprites[SpriteId.xC4_THIEF]
        thief.hp = 4  # Almost no health.
        # TODO: modify weapon damage table so they can actually get hit

    def enable_shadow_bees(self) -> None:
        bees = self.context.sprites[SpriteId.x79_BEE]
        bees.set_invisible()
        bees.set_invincible()

    def save(self, write: Callable[[int, int], None]) -> None:
        self.context.assert_loaded()

        read_overworld_areas(self.context.overworld_areas, write)
        write_sprites(self.context.sprites, write)
        save_rooms(self.context.dungeon_rooms, write)
        write_sprite_blocksets(self.context.sprite_blocksets, write)
