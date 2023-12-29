from typing import Dict

from .LocalRom import LocalRom
from ..Model import SpriteId, Sprite, SpriteSubclassId


def _read_sprite(rom: LocalRom, id: SpriteId) -> Sprite:
    if id >= 0xF3:
        # Addresses above this value are overlords, this is not applicable.
        return Sprite(id=id)
    # Read from health points table. 0xFF is interpreted as None.
    hp = rom.read_snes_address(rom.sprite_health_address + id)

    settings_0_byte = rom.read_snes_address(rom.sprite_setting_0_address + id)
    display_allocation = settings_0_byte & 0b0001_1111
    collisions_alt = bool(settings_0_byte & 0b0010_0000)
    master_sword_only = bool(settings_0_byte & 0b0100_0000)
    harmless = bool(settings_0_byte & 0b1000_0000)

    # The damage settings of a sprite
    damage_byte = rom.read_snes_address(rom.sprite_damage_address + id)
    subclass = SpriteSubclassId(damage_byte & 0b0000_1111)
    boss_prep_preserved = bool(damage_byte & 0b0001_0000)
    immune_to_powder = bool(damage_byte & 0b0010_0000)
    high_priority = bool(damage_byte & 0b0100_0000)
    ignore_recoil_collision = bool(damage_byte & 0b1000_0000)

    # Draw flags and imperviousness
    draw_byte = rom.read_snes_address(rom.sprite_setting_3_address + id)
    name_table = bool(draw_byte & 0b0000_0001)
    palette = (draw_byte & 0b0000_1110) >> 1
    draw_shadow = draw_byte & 0b0001_0000
    big_shadow = draw_byte & 0b0010_0000
    impervious = draw_byte & 0b0100_0000
    custom_death_animation = draw_byte & 0b1000_0000

    # Collision, kill room, killed off screen, hitbox
    settings_4_byte = rom.read_snes_address(rom.sprite_setting_4_address + id)
    hitbox = settings_4_byte & 0b0001_1111
    preserved_offscreen = bool(settings_4_byte & 0b0010_0000)
    stasis = bool(settings_4_byte & 0b0100_0000)
    collision_on_single_layer = bool(settings_4_byte & 0b1000_0000)

    # Settings group 5
    settings_5_byte = rom.read_snes_address(rom.sprite_setting_5_address + id)
    allow_pits = bool(settings_5_byte & 0b0000_0001)
    boss_death_animation = bool(settings_5_byte & 0b0000_0010)
    slashable = bool(settings_5_byte & 0b0000_0100)
    deflect_arrows = bool(settings_5_byte & 0b0000_1000)
    tile_hitbox = (settings_5_byte & 0b1111_0000) >> 4

    return Sprite(
        id=id,
        ignore_recoil_collision=ignore_recoil_collision,
        high_priority=high_priority,
        immune_to_powder=immune_to_powder,
        boss_prep_preserved=boss_prep_preserved,
        subclass=subclass,
        hp=hp,
        display_allocation=display_allocation,
        collisions_alt=collisions_alt,
        master_sword_only=master_sword_only,
        harmless=harmless,
        name_table=name_table,
        palette=palette,
        draw_shadow=draw_shadow,
        big_shadow=big_shadow,
        impervious=impervious,
        custom_death_animation=custom_death_animation,
        hitbox=hitbox,
        preserved_offscreen=preserved_offscreen,
        statis=stasis,
        collision_on_single_layer=collision_on_single_layer,
        allow_pits=allow_pits,
        boss_death_animation=boss_death_animation,
        slashable=slashable,
        deflect_arrows=deflect_arrows,
        tile_hitbox=tile_hitbox,
    )


def read_sprites(rom: LocalRom) -> Dict[SpriteId, Sprite]:
    return {id: _read_sprite(rom, id) for id in list(SpriteId)}


def write_sprite_settings(rom: LocalRom, sprite_dict: Dict[SpriteId, Sprite]) -> None:
    for id, sprite in sprite_dict.items():
        if sprite.display_allocation != None:
            rom.write_snes_address(
                rom.sprite_setting_0_address + id,
                (sprite.display_allocation)
                | (0b10_0000 if sprite.collisions_alt else 0)
                | (0b100_0000 if sprite.master_sword_only else 0)
                | (0b1000_0000 if sprite.harmless else 0),
            )

        if sprite.hp != None:
            rom.write_snes_address(rom.sprite_health_address + id, sprite.hp)

        if sprite.subclass != None:
            rom.write_snes_address(
                rom.sprite_damage_address + id,
                (sprite.subclass)
                | (0b1_0000 if sprite.boss_prep_preserved else 0)
                | (0b10_0000 if sprite.immune_to_powder else 0)
                | (0b100_0000 if sprite.high_priority else 0)
                | (0b1000_0000 if sprite.ignore_recoil_collision else 0),
            )
        if sprite.palette != None:
            rom.write_snes_address(
                rom.sprite_setting_3_address + id,
                (sprite.name_table)
                | (sprite.palette << 1)
                | (0b1_0000 if sprite.draw_shadow else 0)
                | (0b10_0000 if sprite.big_shadow else 0)
                | (0b100_0000 if sprite.impervious else 0)
                | (0b1000_0000 if sprite.custom_death_animation else 0),
            )

        if sprite.hitbox != None:
            rom.write_snes_address(
                rom.sprite_setting_4_address + id,
                (sprite.hitbox)
                | (0b10_0000 if sprite.preserved_offscreen else 0)
                | (0b100_0000 if sprite.statis else 0)
                | (0b1000_0000 if sprite.collision_on_single_layer else 0),
            )

        if sprite.tile_hitbox != None:
            rom.write_snes_address(
                rom.sprite_setting_5_address + id,
                (0b0000_0001 if sprite.allow_pits else 0)
                | (0b0000_0010 if sprite.boss_death_animation else 0)
                | (0b0000_0100 if sprite.slashable else 0)
                | (0b0000_1000 if sprite.deflect_arrows else 0)
                | (sprite.tile_hitbox << 4),
            )
