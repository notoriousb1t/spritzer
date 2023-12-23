from random import Random
from typing import Callable

from .Model.Sprites import SpriteId
from .Rom import (
    LocalRom,
    get_local_rom,
    read_dungeon_rooms,
    write_dungeon_rooms,
    write_overworld_areas,
    read_overworld_areas,
    read_sprite_blocksets,
    write_sprite_blocksets,
    read_sprites,
    write_sprite_settings,
)
from .Transform.Context import Context
from .Transform.DungeonShuffle import reroll_dungeons
from .Transform.OverworldShuffle import reroll_overworld


class Spritzer:
    context: Context
    rom: LocalRom

    def __init__(self, random: Random) -> None:
        self.context = Context(random=random)
        self.rom = None

    def load(self, read: Callable[[int], int]) -> None:
        self.context.assert_unloaded()

        self.rom = get_local_rom(read)
        self.context.sprite_blocksets = read_sprite_blocksets(self.rom)
        self.context.sprites = read_sprites(self.rom)
        self.context.overworld_areas = read_overworld_areas(self.rom)
        self.context.dungeon_rooms = read_dungeon_rooms(self.rom)
        self.context.loaded = True

        areas = list(self.context.overworld_areas.values())
        areas.sort(key=lambda it: it.id)

    def enable_sprite_shuffle_simple(self) -> None:
        self.context.assert_loaded()

        reroll_overworld(self.context)
        reroll_dungeons(self.context)

    def enable_sprite_shuffle_dungeonssimple(self) -> None:
        self.context.assert_loaded()

        reroll_dungeons(self.context)

    def enable_killable_thieves(self) -> None:
        self.context.assert_loaded()

        thief = self.context.sprites[SpriteId.xC4_THIEF]
        thief.hp = 4  # Almost no health.
        # TODO: modify weapon damage table so they can actually get hit

    def enable_shadow_bees(self) -> None:        
        self.context.assert_loaded()

        bees = self.context.sprites[SpriteId.x79_BEE]
        bees.set_invisible()
        bees.set_invincible()

    def save(self, write: Callable[[int, int], None]) -> None:
        self.context.assert_loaded()

        self.rom.enable_write_mode(write)
        write_sprite_settings(self.rom, self.context.sprites)
        write_sprite_blocksets(self.rom, self.context.sprite_blocksets)
        write_overworld_areas(self.rom, self.context.overworld_areas)
        write_dungeon_rooms(self.rom, self.context.dungeon_rooms)
