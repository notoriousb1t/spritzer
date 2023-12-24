from random import Random
from typing import Callable, List

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
from .Transform import (
    Context,
    patch_shadow_bees,
    patch_thief_killable,
    reroll_dungeon_bosses,
    reroll_dungeon_palette,
    reroll_dungeon_tilesets,
    reroll_dungeon_sprites,
    reroll_overworld,
)


class Spritzer:
    _context: Context
    _rom: LocalRom

    transform_list: List[Callable[[Context], None]] = list()
    """Add functions to perform logic on this game."""

    def __init__(self, random: Random) -> None:
        self._context = Context(random=random)
        self._rom = None

    def load(self, read: Callable[[int], int]) -> None:
        self._context.assert_unloaded()

        self._rom = get_local_rom(read)
        self._context.sprite_blocksets = read_sprite_blocksets(self._rom)
        self._context.sprites = read_sprites(self._rom)
        self._context.overworld_areas = read_overworld_areas(self._rom)
        self._context.dungeon_rooms = read_dungeon_rooms(self._rom)
        self._context.loaded = True

    def add_dungeon_palette_swap(self) -> None:
        self.transform_list.append(reroll_dungeon_palette)

    def add_tileset_swap(self) -> None:
        self.transform_list.append(reroll_dungeon_tilesets)

    def add_sprite_shuffle_simple(self) -> None:
        self.transform_list.append(reroll_overworld)
        self.transform_list.append(reroll_dungeon_sprites)
        self.transform_list.append(reroll_dungeon_bosses)

    def add_sprite_shuffle_dungeonssimple(self) -> None:
        self.transform_list.append(reroll_dungeon_sprites)
        self.transform_list.append(reroll_dungeon_bosses)

    def add_killable_thieves(self) -> None:
        self.transform_list.append(patch_thief_killable)

    def add_shadow_bees(self) -> None:
        self.transform_list.append(patch_shadow_bees)

    def save(self, write: Callable[[int, int], None]) -> None:
        self._context.assert_loaded()

        self._rom.set_locked()
        for transform in self.transform_list:
            transform(self._context)

        self._rom.start_write(write)
        write_sprite_settings(self._rom, self._context.sprites)
        write_sprite_blocksets(self._rom, self._context.sprite_blocksets)
        write_overworld_areas(self._rom, self._context.overworld_areas)
        write_dungeon_rooms(self._rom, self._context.dungeon_rooms)
