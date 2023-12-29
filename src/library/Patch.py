from attr import dataclass
from random import Random
from typing import Callable, List

from library.Rom import (
    get_local_rom,
    read_damage_table,
    read_dungeon_rooms,
    read_overworld_areas,
    read_sprites,
    read_spritesets,
    read_sprite_subclasses,
    RomMode,
    write_damage_table,
    write_dungeon_rooms,
    write_overworld_areas,
    write_sprite_blocksets,
    write_sprite_settings,
    write_sprite_subclasses,
)
from library.Transform import (
    Context,
    patch_shadow_bees,
    patch_thief_killable,
    reroll_dungeon_bosses,
    reroll_dungeon_palette,
    reroll_dungeon_sprites,
    reroll_dungeon_tilesets,
    reroll_lost_woods_mushroom,
    reroll_overworld,
    patch_invulnerable_sprites,
)


@dataclass
class Options:
    seed: str = None
    boss_shuffle = False
    dungeon_palette_shuffle = False
    dungeon_sprite_shuffle = False
    dungeon_tileset_shuffle = False
    killable_thieves = False
    mushroom_shuffle = False
    overworld_sprite_shuffle = False
    shadow_bees = False


def patch_buffer(buffer: bytearray, options: Options, random=Random()) -> None:
    """Patch a buffer containing Zelda3. It must have the smc header removed."""
    # Setup all transforms. The order is signficant.
    transform_list: List[Callable[[Context], None]] = list()
    transform_list.append(patch_invulnerable_sprites)
    if options.killable_thieves:
        transform_list.append(patch_thief_killable)
    if options.shadow_bees:
        transform_list.append(patch_shadow_bees)
    if options.mushroom_shuffle:
        transform_list.append(reroll_lost_woods_mushroom)
    if options.dungeon_tileset_shuffle:
        transform_list.append(reroll_dungeon_tilesets)
    if options.dungeon_palette_shuffle:
        transform_list.append(reroll_dungeon_palette)
    if options.boss_shuffle:
        transform_list.append(reroll_dungeon_bosses)
    if options.dungeon_sprite_shuffle:
        transform_list.append(reroll_dungeon_sprites)
    if options.overworld_sprite_shuffle:
        transform_list.append(reroll_overworld)

    # Read the rom and load all models.
    rom = get_local_rom(buffer)
    context = Context(random=random)
    context.damage_table = read_damage_table(rom)

    context.sprite_subclasses = read_sprite_subclasses(rom)
    context.spritesets = read_spritesets(rom)
    context.sprites = read_sprites(rom)
    context.overworld_areas = read_overworld_areas(rom)
    context.dungeon_rooms = read_dungeon_rooms(rom)
    context.loaded = True

    for id, val in context.sprites.items():
        print(f"{val.subclass if val.subclass != None else 'XX'} {id}")

    for id, val in context.sprite_subclasses.items():
        print(f"{id} {val}")

    # Lock the rom and apply all transformations.
    rom.set_mode(RomMode.LOCKED)
    for transform in transform_list:
        transform(context)

    # Write the data back to the ROM.
    rom.set_mode(RomMode.WRITE)

    write_sprite_subclasses(rom, context.sprite_subclasses)
    write_damage_table(rom, context.damage_table)
    write_sprite_settings(rom, context.sprites)
    write_sprite_blocksets(rom, context.spritesets)
    write_overworld_areas(rom, context.overworld_areas)
    write_dungeon_rooms(rom, context.dungeon_rooms)

    # Write CRC to the ROM.
    rom.set_mode(RomMode.CRC)
    rom.write_crc()


def patch_file(options: Options, input_path: str, output_path: str) -> None:
    """Patch a file. This is intended for the included GUI's use"""
    with open(input_path, "rb") as stream:
        buffer = bytearray(stream.read())
        if len(buffer) % 0x400 == 0x200:
            buffer = buffer[0x200:]

    random = Random()
    if options.seed:
        # Makes the seed determistic
        random.seed(options.seed)
    patch_buffer(buffer, options, random)

    with open(output_path, "wb") as outfile:
        outfile.write(buffer)
