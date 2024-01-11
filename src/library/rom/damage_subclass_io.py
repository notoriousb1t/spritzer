from typing import Dict

from library.model.damage_subclass import DamageSubclass
from library.model.damage_subclass_id import DamageSubclassId
from library.rom.local_rom import LocalRom


def read_damage_subclasses(rom: LocalRom) -> Dict[DamageSubclassId, DamageSubclass]:
    subclasses: Dict[DamageSubclassId, DamageSubclass] = {}
    # SNES 0x36F33 - Damage ptr of weapons - 4 bytes (also determines which enemies you can kill with, and which enemies will be one-hit-ko)
    for id in list(DamageSubclassId):
        subclasses[id] = DamageSubclass(
            boomerang_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0x0
            ),
            sword1_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0x1
            ),
            sword2_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0x2
            ),
            sword3_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0x3
            ),
            sword4_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0x4
            ),
            sword5_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0x5
            ),
            arrow1_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0x6
            ),
            hookshot_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0x7
            ),
            bomb_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0x8
            ),
            arrow2_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0x9
            ),
            powder_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0xA
            ),
            fire_rod_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0xB
            ),
            ice_rod_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0xC
            ),
            bombos_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0xD
            ),
            ether_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0xE
            ),
            quake_damage=rom.read_snes_address(
                snes_address=rom.weapon_damage_snes + (id * 8) + 0xF
            ),
        )

    return subclasses


def write_damage_subclasses(
    rom: LocalRom, subclasses: Dict[DamageSubclassId, DamageSubclass]
) -> None:
    for id, subclass in subclasses.items():
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0x0,
            value=subclass.boomerang_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0x1,
            value=subclass.sword1_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0x2,
            value=subclass.sword2_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0x3,
            value=subclass.sword3_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0x4,
            value=subclass.sword4_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0x5,
            value=subclass.sword5_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0x6,
            value=subclass.arrow1_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0x7,
            value=subclass.hookshot_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0x8,
            value=subclass.bomb_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0x9,
            value=subclass.arrow2_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0xA,
            value=subclass.powder_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0xB,
            value=subclass.fire_rod_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0xC,
            value=subclass.ice_rod_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0xD,
            value=subclass.bombos_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0xE,
            value=subclass.ether_damage,
        )
        rom.write_snes_address(
            snes_address=rom.weapon_damage_snes + (id * 8) + 0xF,
            value=subclass.quake_damage,
        )
