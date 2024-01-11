from random import Random
from typing import Callable, List
from library.model.sprite_sheet_id import create_spriteset_dict
from library.options.options import Options
from library.options.overworld_enemy_shuffle import OverworldEnemyShuffle
from library.options.underworld_enemy_shuffle import UnderworldEnemyShuffle
from library.rom.damage_io import read_damage_table, write_damage_table
from library.rom.damage_subclass_io import (
    read_damage_subclasses,
    write_damage_subclasses,
)
from library.rom.local_rom import LocalRom, get_local_rom
from library.rom.overworld_io import read_overworld_areas, write_overworld_areas
from library.rom.rom_mode import RomMode
from library.rom.sprite_io import read_sprite_settings, write_sprite_settings
from library.rom.spriteset_io import read_spritesets, write_spritesets
from library.rom.underworld_io import read_underworld_rooms, write_underworld_rooms
from library.transform.boss_shuffle import reroll_underworld_bosses
from library.model.model import Model
from library.transform.killable_sprite import patch_thief_killable
from library.transform.mushroom_shuffle import reroll_lost_woods_mushroom
from library.transform.overworld_inversion import invert_world
from library.transform.overworld_shuffle import reroll_overworld_enemies
from library.transform.shadow_bees import patch_shadow_bees
from library.transform.sprite_choices import (
    preprocess_full_overworld_choices,
    preprocess_full_underworld_choices,
    preprocess_simple_overworld_choices,
    preprocess_simple_underworld_choices,
)
from library.transform.sprite_expansion import expand_overworld_sprite_pool
from library.transform.spriteset_choices import create_free_overworld_spriteset_list
from library.transform.underworld_palette_shuffle import reroll_underworld_palette
from library.transform.underworld_shuffle import reroll_underworld_enemies
from library.transform.underworld_tileset_shuffle import reroll_underworld_blocksets
from library.transform.vanilla_fixes import patch_invulnerable_sprites


def patch_buffer(
    buffer: bytearray,
    options: Options,
    random: Random = Random(),
) -> None:
    """Patch a buffer containing Zelda3. It must have the smc header removed."""
    # Setup all transforms. The order is signficant.
    transform_list: List[Callable[[Model], None]] = list()
    preprocess_list: List[Callable[[Model], None]] = list()

    if options.killable_thieves:
        transform_list.append(patch_thief_killable)

    if options.shadow_bees:
        transform_list.append(patch_shadow_bees)

    if options.mushroom_shuffle:
        transform_list.append(reroll_lost_woods_mushroom)

    if options.underworld_tileset_shuffle:
        transform_list.append(reroll_underworld_blocksets)

    if options.underworld_palette_shuffle:
        transform_list.append(reroll_underworld_palette)

    if options.boss_shuffle:
        transform_list.append(reroll_underworld_bosses)

    if options.underworld_enemy_shuffle == UnderworldEnemyShuffle.SIMPLE:
        preprocess_list.append(preprocess_simple_underworld_choices)
        preprocess_list.append(patch_invulnerable_sprites)
        transform_list.append(reroll_underworld_enemies)
    elif options.underworld_enemy_shuffle == UnderworldEnemyShuffle.FULL:
        preprocess_list.append(preprocess_full_underworld_choices)
        preprocess_list.append(patch_invulnerable_sprites)
        transform_list.append(reroll_underworld_enemies)
    elif options.underworld_enemy_shuffle == UnderworldEnemyShuffle.CHAOS:
        preprocess_list.append(preprocess_full_underworld_choices)
        preprocess_list.append(patch_invulnerable_sprites)
        transform_list.append(reroll_underworld_enemies)
    elif options.underworld_enemy_shuffle == UnderworldEnemyShuffle.INSANITY:
        preprocess_list.append(preprocess_full_underworld_choices)
        preprocess_list.append(patch_invulnerable_sprites)
        transform_list.append(reroll_underworld_enemies)

    if options.overworld_enemy_shuffle == OverworldEnemyShuffle.SIMPLE:
        preprocess_list.append(preprocess_simple_overworld_choices)
        transform_list.append(reroll_overworld_enemies)
    elif options.overworld_enemy_shuffle == OverworldEnemyShuffle.INVERTED:
        preprocess_list.append(invert_world)
        preprocess_list.append(preprocess_full_overworld_choices)
        preprocess_list.append(expand_overworld_sprite_pool)
        transform_list.append(reroll_overworld_enemies)
    elif options.overworld_enemy_shuffle == OverworldEnemyShuffle.FULL:
        preprocess_list.append(preprocess_full_overworld_choices)
        preprocess_list.append(expand_overworld_sprite_pool)
        transform_list.append(reroll_overworld_enemies)
    elif options.overworld_enemy_shuffle == OverworldEnemyShuffle.CHAOS:
        preprocess_list.append(preprocess_full_overworld_choices)
        preprocess_list.append(expand_overworld_sprite_pool)
        transform_list.append(reroll_overworld_enemies)
    elif options.overworld_enemy_shuffle == OverworldEnemyShuffle.INSANITY:
        preprocess_list.append(preprocess_full_overworld_choices)
        preprocess_list.append(expand_overworld_sprite_pool)
        transform_list.append(reroll_overworld_enemies)

    rom: LocalRom = get_local_rom(buffer=buffer)
    context = Model(
        seed=options.seed,
        overworld_balancing=options.overworld_balancing,
        underworld_balancing=options.underworld_balancing,
        damage_table=read_damage_table(rom=rom),
        sprite_subclasses=read_damage_subclasses(rom=rom),
        spritesets=read_spritesets(rom=rom),
        sprites=read_sprite_settings(rom=rom),
        overworld_areas=read_overworld_areas(rom=rom),
        underworld_rooms=read_underworld_rooms(rom=rom),
        spritesheet_sprites=create_spriteset_dict(),
    )

    # Perform preprocessing
    context.unused_spritesets = create_free_overworld_spriteset_list(context=context)
    if preprocess_list:
        for preprocessor in preprocess_list:
            preprocessor(context)

    # Remove reading to enforce this is transactional.
    rom.set_mode(mode=RomMode.LOCKED)
    for transform in transform_list:
        transform(context)

    # Write the data back to the ROM.
    rom.set_mode(mode=RomMode.WRITE)
    write_damage_subclasses(rom=rom, subclasses=context.sprite_subclasses)
    write_damage_table(rom=rom, damage_table=context.damage_table)
    write_sprite_settings(rom=rom, sprite_dict=context.sprites)
    write_spritesets(rom=rom, blocksets=context.spritesets)
    write_overworld_areas(rom=rom, overworld_areas=context.overworld_areas)
    write_underworld_rooms(rom=rom, dungeon_room_dict=context.underworld_rooms)
    # Write CRC to the ROM.
    rom.set_mode(mode=RomMode.CRC)
    rom.write_crc()
    # NOTE: uncomment to see all deltas
    # for delta in rom.get_deltas():
    #     print(delta)


def patch_file(
    options: Options,
    input_path: str,
    output_path: str,
) -> None:
    """Patch a file. This is intended for the included GUI's use"""
    with open(file=input_path, mode="rb") as stream:
        buffer = bytearray(stream.read())
        if len(buffer) % 0x400 == 0x200:
            buffer: bytearray = buffer[0x200:]

    random = Random()
    if options.seed:
        # Makes the seed determistic
        random.seed(a=options.seed)
    patch_buffer(buffer=buffer, options=options, random=random)

    with open(file=output_path, mode="wb") as outfile:
        outfile.write(buffer)
