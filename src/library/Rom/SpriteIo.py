from typing import Dict

from .LocalRom import LocalRom
from ..Model import SpriteId, Sprite, SpriteFlags3, SpriteFlags0, SpriteSubclassId


def _read_sprite(rom: LocalRom, id: SpriteId) -> Sprite:
    if id >= 0xF3:
        # Addresses above this value are overlords, this is not applicable.
        return Sprite(id=id)
    # Read from health points table. 0xFF is interpreted as None.
    hp = rom.read_snes_address(rom.sprite_health_address + id)

    # The damage settings of a sprite
    damage_byte = rom.read_snes_address(rom.sprite_damage_address + id)
    subclass = SpriteSubclassId(damage_byte & 0b1111)
    boss_prep_preserved = bool(damage_byte & 0b1_0000)
    immune_to_powder = bool(damage_byte & 0b10_0000)
    high_priority = bool(damage_byte & 0b100_0000)
    ignore_recoil_collision = bool(damage_byte & 0b1000_0000)

    flags_0 = SpriteFlags0(rom.read_snes_address(rom.sprite_setting_address + id))
    flags_3 = SpriteFlags3(rom.read_snes_address(rom.sprite_setting_3_address + id))
    settings_4 = rom.read_snes_address(rom.sprite_setting_4_address + id)
    settings_5 = rom.read_snes_address(rom.sprite_setting_5_address + id)

    return Sprite(
        id=id,
        ignore_recoil_collision=ignore_recoil_collision,
        high_priority=high_priority,
        immune_to_powder=immune_to_powder,
        boss_prep_preserved=boss_prep_preserved,
        subclass=subclass,
        hp=hp,
        flags_0=flags_0,
        flags_3=flags_3,
        settings_4=settings_4,
        settings_5=settings_5,
    )


def read_sprites(rom: LocalRom) -> Dict[SpriteId, Sprite]:
    return {id: _read_sprite(rom, id) for id in list(SpriteId)}


def write_sprite_settings(rom: LocalRom, sprite_dict: Dict[SpriteId, Sprite]) -> None:
    for id, sprite in sprite_dict.items():
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

        if sprite.flags_0 != None:
            rom.write_snes_address(rom.sprite_setting_address + id, sprite.flags_0)
        if sprite.flags_3 != None:
            rom.write_snes_address(rom.sprite_setting_3_address + id, sprite.flags_3)
        if sprite.settings_4 != None:
            rom.write_snes_address(rom.sprite_setting_4_address + id, sprite.settings_4)
        if sprite.settings_5 != None:
            rom.write_snes_address(rom.sprite_setting_5_address + id, sprite.settings_5)
