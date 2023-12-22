from typing import Callable, Dict

from ..Model import SpriteId, Sprite, Flags0, Flags3


_setting_0_address = 0x06B080
_sprite_health_address = _setting_0_address + (0xF3 * 1)
_sprite_damage_address = _setting_0_address + (0xF3 * 2)
_setting_3_address = _setting_0_address + (0xF3 * 3)
_setting_4_address = _setting_0_address + (0xF3 * 4)
_setting_5_address = _setting_0_address + (0xF3 * 5)


def _read_sprite(id: SpriteId, read_address: Callable[[int], int]) -> Sprite:
    if id >= 0xF3:
        # Addresses above this value are overlords, this is not applicable.
        return Sprite(id=id)
    # Read from health points table. 0xFF is interpreted as None.
    hp = read_address(_sprite_health_address + id)
    # The amount of damage the sprite does to link.
    damage = read_address(_sprite_damage_address + id)
    flags_0 = Flags0(read_address(_setting_0_address + id))
    flags_3 = Flags3(read_address(_setting_3_address + id))
    settings_4 = read_address(_setting_4_address + id)
    settings_5 = read_address(_setting_5_address + id)

    return Sprite(
        id=id,
        damage=damage,
        hp=hp,
        flags_0=flags_0,
        flags_3=flags_3,
        settings_4=settings_4,
        settings_5=settings_5,
    )


def read_sprites(read_address: Callable[[int], int]) -> Dict[SpriteId, Sprite]:
    return {id: _read_sprite(id, read_address) for id in list(SpriteId)}


def write_sprites(
    sprite_dict: Dict[SpriteId, Sprite],
    write_address: Callable[[int, int], None],
) -> None:
    for id, sprite in sprite_dict.items():
        if sprite.hp != None:
            write_address(_sprite_health_address + id, sprite.hp)
        if sprite.damage != None:
            write_address(_sprite_damage_address + id, sprite.damage)
        if sprite.flags_0 != None:
            write_address(_setting_0_address + id, sprite.flags_0)
        if sprite.flags_3 != None:
            write_address(_setting_3_address + id, sprite.flags_3)
        if sprite.settings_4 != None:
            write_address(_setting_4_address + id, sprite.settings_4)
        if sprite.settings_5 != None:
            write_address(_setting_5_address + id, sprite.settings_5)
