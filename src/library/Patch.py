from random import Random
from typing import Callable, List

from .Options import Options, OverworldEnemyShuffle, DungeonEnemyShuffle

from .Model import create_spriteset_dict, create_free_spriteset_list
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
    write_spritesets,
    write_sprite_settings,
    write_sprite_subclasses,
)
from library.Transform import (
    Context,
    preprocess_simple_overworld_choices,
    preprocess_simple_dungeon_choices,
    preprocess_full_overworld_choices,
    preprocess_full_dungeon_choices,
    expand_overworld_sprite_pool,
    invert_world,
    patch_shadow_bees,
    patch_thief_killable,
    reroll_dungeon_bosses,
    reroll_dungeon_palette,
    reroll_dungeon_enemies,
    reroll_dungeon_blocksets,
    reroll_lost_woods_mushroom,
    reroll_overworld_enemies,
    patch_invulnerable_sprites,
)


def patch(
    buffer: bytearray,
    random: Random,
    preprocess_list: List[Callable[[Context], None]],
    transform_list: List[Callable[[Context], None]],
) -> None:
    rom = get_local_rom(buffer)
    context = Context(random=random)
    # Read the rom and load all models.
    context.damage_table = read_damage_table(rom)
    context.sprite_subclasses = read_sprite_subclasses(rom)
    context.spritesets = read_spritesets(rom)
    context.sprites = read_sprites(rom)
    context.overworld_areas = read_overworld_areas(rom)
    context.dungeon_rooms = read_dungeon_rooms(rom)
    context.unused_spritesets = create_free_spriteset_list()

    # Perform preprocessing
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
    write_dungeon_rooms(rom, context.dungeon_rooms)
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

    if options.dungeon_tileset_shuffle:
        transform_list.append(reroll_dungeon_blocksets)

    if options.dungeon_palette_shuffle:
        transform_list.append(reroll_dungeon_palette)

    if options.boss_shuffle:
        transform_list.append(reroll_dungeon_bosses)

    if options.dungeon_enemy_shuffle == DungeonEnemyShuffle.SIMPLE:
        preprocess_list.append(preprocess_simple_dungeon_choices)
        preprocess_list.append(patch_invulnerable_sprites)
        transform_list.append(reroll_dungeon_enemies)
    elif options.dungeon_enemy_shuffle == DungeonEnemyShuffle.FULL:
        preprocess_list.append(preprocess_full_dungeon_choices)
        preprocess_list.append(patch_invulnerable_sprites)
        transform_list.append(reroll_dungeon_enemies)
    elif options.dungeon_enemy_shuffle == DungeonEnemyShuffle.CHAOS:
        preprocess_list.append(preprocess_full_dungeon_choices)
        preprocess_list.append(patch_invulnerable_sprites)
        transform_list.append(reroll_dungeon_enemies)
    elif options.dungeon_enemy_shuffle == DungeonEnemyShuffle.INSANITY:
        preprocess_list.append(preprocess_full_dungeon_choices)
        preprocess_list.append(patch_invulnerable_sprites)
        transform_list.append(reroll_dungeon_enemies)

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
