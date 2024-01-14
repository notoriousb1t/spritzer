from typing import Dict

from library.model.sprite import Sprite
from library.model.sprite_id import SpriteId
from library.model.damage_subclass_id import DamageSubclassId
from library.rom.local_rom import LocalRom


def _read_sprite(rom: LocalRom, id: SpriteId) -> Sprite:
    if id >= 0xF3:
        # Addresses above this value are overlords, this is not applicable.
        return Sprite(id=id)

    settings_0_byte: int = rom.read_snes_address(snes_address=rom.sprite_settings0_snes + id)
    display_allocation: int = settings_0_byte & 0b1_1111
    collisions_alt = bool(settings_0_byte & 0b10_0000)
    master_sword_only = bool(settings_0_byte & 0b100_0000)
    harmless = bool(settings_0_byte & 0b1000_0000)

    # Read from health points table. 0xFF is interpreted as None.
    hp: int = rom.read_snes_address(snes_address=rom.sprite_settings1_snes + id)

    # The damage settings of a sprite
    settings_2_byte = rom.read_snes_address(snes_address=rom.sprite_settings2_snes + id)
    subclass = DamageSubclassId(value=settings_2_byte & 0b1111)
    boss_prep_preserved = bool(settings_2_byte & 0b1_0000)
    immune_to_powder = bool(settings_2_byte & 0b10_0000)
    high_priority = bool(settings_2_byte & 0b0100_0000)
    ignore_recoil_collision = bool(settings_2_byte & 0b1000_0000)

    # Draw flags and imperviousness
    settings_3_byte: int = rom.read_snes_address(snes_address=rom.sprite_settings3_snes + id)
    name_table = bool(settings_3_byte & 0b1)
    palette: int = (settings_3_byte & 0b1110) >> 1
    draw_shadow: bool = bool(settings_3_byte & 0b1_0000)
    big_shadow: bool = bool(settings_3_byte & 0b10_0000)
    impervious: bool = bool(settings_3_byte & 0b100_0000)
    custom_death_animation: bool = bool(settings_3_byte & 0b1000_0000)

    # Collision, kill room, killed off screen, hitbox
    settings_4_byte: int = rom.read_snes_address(snes_address=rom.sprite_settings4_snes + id)
    hitbox: int = settings_4_byte & 0b1_1111
    preserved_offscreen = bool(settings_4_byte & 0b10_0000)
    stasis = bool(settings_4_byte & 0b100_0000)
    collision_on_single_layer = bool(settings_4_byte & 0b1000_0000)

    # Settings group 5
    settings_5_byte: int = rom.read_snes_address(snes_address=rom.sprite_settings5_snes + id)
    allow_pits = bool(settings_5_byte & 0b1)
    boss_death_animation = bool(settings_5_byte & 0b10)
    slashable = bool(settings_5_byte & 0b0100)
    deflect_arrows = bool(settings_5_byte & 0b1000)
    tile_hitbox: int = (settings_5_byte & 0b1111_0000) >> 4

    # Settings group 6
    settings_6_byte: int = rom.read_snes_address(snes_address=rom.sprite_settings6_snes + id)
    prize_pack: int = settings_6_byte & 0b1111
    boss_damage_sfx = bool(settings_6_byte & 0b1_0000)
    blockable = bool(settings_6_byte & 0b10_0000)
    special_water_check = bool(settings_6_byte & 0b100_0000)
    ignore_floor = bool(settings_6_byte & 0b1000_0000)

    # Settings group 7
    settings_7_byte: int = rom.read_snes_address(snes_address=rom.sprite_settings7_snes + id)
    stay_active_offscreen = bool(settings_7_byte & 0b1)
    die_off_screen = bool(settings_7_byte & 0b10)
    moveable_unused = bool(settings_7_byte & 0b100)
    projectiles_unused = bool(settings_7_byte & 0b1000)
    projectile = bool(settings_7_byte & 0b1_0000)
    immune_to_sword_hammer = bool(settings_7_byte & 0b10_0000)
    immune_to_arrows = bool(settings_7_byte & 0b100_0000)
    prevent_permadeath = bool(settings_7_byte & 0b1000_0000)

    return Sprite(
        id=id,
        allow_pits=allow_pits,
        big_shadow=big_shadow,
        blockable=blockable,
        boss_damage_sfx=boss_damage_sfx,
        boss_death_animation=boss_death_animation,
        boss_prep_preserved=boss_prep_preserved,
        collision_on_single_layer=collision_on_single_layer,
        collisions_alt=collisions_alt,
        custom_death_animation=custom_death_animation,
        deflect_arrows=deflect_arrows,
        die_off_screen=die_off_screen,
        display_allocation=display_allocation,
        draw_shadow=draw_shadow,
        harmless=harmless,
        high_priority=high_priority,
        hitbox=hitbox,
        hp=hp,
        ignore_floor=ignore_floor,
        ignore_recoil_collision=ignore_recoil_collision,
        immune_to_arrows=immune_to_arrows,
        immune_to_powder=immune_to_powder,
        immune_to_sword_hammer=immune_to_sword_hammer,
        impervious=impervious,
        master_sword_only=master_sword_only,
        moveable_unused=moveable_unused,
        name_table=name_table,
        palette=palette,
        preserved_offscreen=preserved_offscreen,
        prevent_permadeath=prevent_permadeath,
        prize_pack=prize_pack,
        projectile=projectile,
        projectiles_unused=projectiles_unused,
        slashable=slashable,
        special_water_check=special_water_check,
        statis=stasis,
        stay_active_offscreen=stay_active_offscreen,
        subclass=subclass,
        tile_hitbox=tile_hitbox,
    )


def read_sprite_settings(rom: LocalRom) -> Dict[SpriteId, Sprite]:
    return {id: _read_sprite(rom, id) for id in list(SpriteId)}


def write_sprite_settings(rom: LocalRom, sprite_dict: Dict[SpriteId, Sprite]) -> None:
    for id, sprite in sprite_dict.items():
        if id >= 0xF3:
            # Skip overlords, they aren't real Sprites.
            continue

        rom.write_snes_address(
            snes_address=rom.sprite_settings0_snes + id,
            value=(sprite.display_allocation)
            | (0b10_0000 if sprite.collisions_alt else 0)
            | (0b100_0000 if sprite.master_sword_only else 0)
            | (0b1000_0000 if sprite.harmless else 0),
        )
        rom.write_snes_address(snes_address=rom.sprite_settings1_snes + id, value=sprite.hp)
        rom.write_snes_address(
            snes_address=rom.sprite_settings2_snes + id,
            value=(sprite.subclass)
            | (0b1_0000 if sprite.boss_prep_preserved else 0)
            | (0b10_0000 if sprite.immune_to_powder else 0)
            | (0b100_0000 if sprite.high_priority else 0)
            | (0b1000_0000 if sprite.ignore_recoil_collision else 0),
        )
        rom.write_snes_address(
            snes_address=rom.sprite_settings3_snes + id,
            value=(sprite.name_table)
            | (sprite.palette << 1)
            | (0b1_0000 if sprite.draw_shadow else 0)
            | (0b10_0000 if sprite.big_shadow else 0)
            | (0b100_0000 if sprite.impervious else 0)
            | (0b1000_0000 if sprite.custom_death_animation else 0),
        )
        rom.write_snes_address(
            snes_address=rom.sprite_settings4_snes + id,
            value=(sprite.hitbox & 0b11111)
            | (0b10_0000 if sprite.preserved_offscreen else 0)
            | (0b100_0000 if sprite.statis else 0)
            | (0b1000_0000 if sprite.collision_on_single_layer else 0),
        )
        rom.write_snes_address(
            snes_address=rom.sprite_settings5_snes + id,
            value=(0b1 if sprite.allow_pits else 0)
            | (0b10 if sprite.boss_death_animation else 0)
            | (0b100 if sprite.slashable else 0)
            | (0b1000 if sprite.deflect_arrows else 0)
            | ((sprite.tile_hitbox & 0b1111) << 4),
        )
        rom.write_snes_address(
            snes_address=rom.sprite_settings6_snes + id,
            value=(sprite.prize_pack & 0b1111)
            | (0b1_0000 if sprite.boss_damage_sfx else 0)
            | (0b10_0000 if sprite.blockable else 0)
            | (0b100_0000 if sprite.special_water_check else 0)
            | (0b1000_0000 if sprite.ignore_floor else 0),
        )
        rom.write_snes_address(
            snes_address=rom.sprite_settings7_snes + id,
            value=(0b1 if sprite.stay_active_offscreen else 0)
            | (0b10 if sprite.die_off_screen else 0)
            | (0b100 if sprite.moveable_unused else 0)
            | (0b1000 if sprite.projectiles_unused else 0)
            | (0b1_0000 if sprite.projectile else 0)
            | (0b10_0000 if sprite.immune_to_sword_hammer else 0)
            | (0b100_0000 if sprite.immune_to_arrows else 0)
            | (0b1000_0000 if sprite.prevent_permadeath else 0),
        )
