from typing import Dict

from library.Model.SpriteSubclass import SpriteSubclass
from library.Model.SpriteSubclassId import SpriteSubclassId
from library.Rom.LocalRom import LocalRom


def read_sprite_subclasses(rom: LocalRom) -> Dict[SpriteSubclassId, SpriteSubclass]:
    subclasses: Dict[SpriteSubclassId, SpriteSubclass] = {}
    # SNES 0x36F33 - Damage ptr of weapons - 4 bytes (also determines which enemies you can kill with, and which enemies will be one-hit-ko)
    for id in list(SpriteSubclassId):
        subclasses[id] = SpriteSubclass(
            boomerang_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0x0
            ),
            sword1_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0x1
            ),
            sword2_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0x2
            ),
            sword3_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0x3
            ),
            sword4_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0x4
            ),
            sword5_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0x5
            ),
            arrow1_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0x6
            ),
            hookshot_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0x7
            ),
            bomb_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0x8
            ),
            arrow2_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0x9
            ),
            powder_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0xA
            ),
            fire_rod_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0xB
            ),
            ice_rod_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0xC
            ),
            bombos_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0xD
            ),
            ether_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0xE
            ),
            quake_damage=rom.read_snes_address(
                rom.weapon_damage_snes + (id * 8) + 0xF
            ),
        )

    return subclasses


def write_sprite_subclasses(
    rom: LocalRom, subclasses: Dict[SpriteSubclassId, SpriteSubclass]
) -> None:
    for id, subclass in subclasses.items():
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0x0,
            subclass.boomerang_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0x1,
            subclass.sword1_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0x2,
            subclass.sword2_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0x3,
            subclass.sword3_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0x4,
            subclass.sword4_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0x5,
            subclass.sword5_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0x6,
            subclass.arrow1_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0x7,
            subclass.hookshot_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0x8,
            subclass.bomb_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0x9,
            subclass.arrow2_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0xA,
            subclass.powder_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0xB,
            subclass.fire_rod_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0xC,
            subclass.ice_rod_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0xD,
            subclass.bombos_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0xE,
            subclass.ether_damage,
        )
        rom.write_snes_address(
            rom.weapon_damage_snes + (id * 8) + 0xF,
            subclass.quake_damage,
        )
