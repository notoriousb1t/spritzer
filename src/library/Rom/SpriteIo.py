from typing import Dict

from .LocalRom import LocalRom
from ..Model import SpriteId, Sprite, Flags0, Flags3


def _read_sprite(rom: LocalRom, id: SpriteId) -> Sprite:
    if id >= 0xF3:
        # Addresses above this value are overlords, this is not applicable.
        return Sprite(id=id)
    # Read from health points table. 0xFF is interpreted as None.
    hp = rom.read_address(rom.sprite_health_address + id)
    # The amount of damage the sprite does to link.
    damage = rom.read_address(rom.sprite_damage_address + id)
    flags_0 = Flags0(rom.read_address(rom.sprite_setting_0_address + id))
    flags_3 = Flags3(rom.read_address(rom.sprite_setting_3_address + id))
    settings_4 = rom.read_address(rom.sprite_setting_4_address + id)
    settings_5 = rom.read_address(rom.sprite_setting_5_address + id)

    return Sprite(
        id=id,
        damage=damage,
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
            rom.write_address(rom.sprite_health_address + id, sprite.hp)
        if sprite.damage != None:
            rom.write_address(rom.sprite_damage_address + id, sprite.damage)
        if sprite.flags_0 != None:
            rom.write_address(rom.sprite_setting_0_address + id, sprite.flags_0)
        if sprite.flags_3 != None:
            rom.write_address(rom.sprite_setting_3_address + id, sprite.flags_3)
        if sprite.settings_4 != None:
            rom.write_address(rom.sprite_setting_4_address + id, sprite.settings_4)
        if sprite.settings_5 != None:
            rom.write_address(rom.sprite_setting_5_address + id, sprite.settings_5)
