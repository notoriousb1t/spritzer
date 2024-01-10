from random import Random
from typing import Callable, List

from library.Model.SpriteSheetId import create_spriteset_dict
from library.Options import Options, OverworldEnemyShuffle, UnderworldEnemyShuffle
from library.Rom.DamageIo import read_damage_table, write_damage_table
from library.Rom.LocalRom import RomMode, get_local_rom
from library.Rom.OverworldIo import read_overworld_areas, write_overworld_areas
from library.Rom.SpriteIo import read_sprites, write_sprite_settings
from library.Rom.SpriteSheetIo import read_spritesets, write_spritesets
from library.Rom.SubclassIo import read_sprite_subclasses, write_sprite_subclasses
from library.Rom.UnderworldIo import read_underworld_rooms, write_underworld_rooms
from library.Transform.BossShuffle import reroll_underworld_bosses
from library.Transform.Context import Balancing, Context
from library.Transform.KillableSprite import patch_thief_killable
from library.Transform.MushroomShuffle import reroll_lost_woods_mushroom
from library.Transform.OverworldInversion import invert_world
from library.Transform.OverworldShuffle import reroll_overworld_enemies
from library.Transform.ShadowBees import patch_shadow_bees
from library.Transform.SpriteChoices import (
    preprocess_full_overworld_choices,
    preprocess_full_underworld_choices,
    preprocess_simple_overworld_choices,
    preprocess_simple_underworld_choices,
)
from library.Transform.SpriteExpansion import expand_overworld_sprite_pool
from library.Transform.SpritesetChoices import create_free_overworld_spriteset_list
from library.Transform.UnderworldPaletteShuffle import reroll_underworld_palette
from library.Transform.UnderworldShuffle import reroll_underworld_enemies
from library.Transform.UnderworldTilesetShuffle import reroll_underworld_blocksets
from library.Transform.VanillaFixes import patch_invulnerable_sprites


def patch(
    buffer: bytearray,
    random: Random,
    preprocess_list: List[Callable[[Context], None]],
    transform_list: List[Callable[[Context], None]],
    overworld_balancing: Balancing,
    underworld_balancing: Balancing,
) -> None:
    rom = get_local_rom(buffer)
    context = Context(random=random)

    # Read the rom and load all models.
    context.damage_table = read_damage_table(rom)
    context.sprite_subclasses = read_sprite_subclasses(rom)
    context.spritesets = read_spritesets(rom)
    context.sprites = read_sprites(rom)
    context.overworld_areas = read_overworld_areas(rom)
    context.underworld_rooms = read_underworld_rooms(rom)
    context.overworld_balancing = overworld_balancing
    context.underworld_balancing = underworld_balancing

    # Perform preprocessing
    context.unused_spritesets = create_free_overworld_spriteset_list(context)
    context.spritesheet_sprites = create_spriteset_dict()
    if preprocess_list:
        for preprocessor in preprocess_list:
            preprocessor(context)

    # Remove reading to enforce this is transactional.
    rom.set_mode(RomMode.LOCKED)
    for transform in transform_list:
        transform(context)

    # Write the data back to the ROM.
    rom.set_mode(RomMode.WRITE)
    write_sprite_subclasses(rom, context.sprite_subclasses)
    write_damage_table(rom, context.damage_table)
    write_sprite_settings(rom, context.sprites)
    write_spritesets(rom, context.spritesets)
    write_overworld_areas(rom, context.overworld_areas)
    write_underworld_rooms(rom, context.underworld_rooms)
    # Write CRC to the ROM.
    rom.set_mode(RomMode.CRC)
    rom.write_crc()
    # NOTE: uncomment to see all deltas
    # for delta in rom.get_deltas():
    #     print(delta)


def patch_buffer(
    buffer: bytearray,
    options: Options,
    random=Random(),
) -> None:
    """Patch a buffer containing Zelda3. It must have the smc header removed."""
    # Setup all transforms. The order is signficant.
    transform_list: List[Callable[[Context], None]] = list()
    preprocess_list: List[Callable[[Context], None]] = list()

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

    patch(
        buffer=buffer,
        random=random,
        preprocess_list=preprocess_list,
        transform_list=transform_list,
        overworld_balancing=options.overworld_balancing,
        underworld_balancing=options.underworld_balancing,
    )


def patch_file(
    options: Options,
    input_path: str,
    output_path: str,
) -> None:
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
