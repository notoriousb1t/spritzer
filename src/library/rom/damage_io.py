from typing import List
from library.model.damage_table import DamageRow, DamageTable
from library.rom.local_rom import LocalRom


def read_damage_table(rom: LocalRom) -> DamageTable:
    link_damage_rows: List[DamageRow] = list()
    for index in range(0, 10):
        # Read Link's damage table.
        link_damage_rows.append(
            DamageRow(
                green_mail=rom.read_snes_address(
                    snes_address=rom.damage_table_snes + (index * 3),
                ),
                blue_mail=rom.read_snes_address(
                    snes_address=rom.damage_table_snes + (index * 3) + 1,
                ),
                red_mail=rom.read_snes_address(
                    snes_address=rom.damage_table_snes + (index * 3) + 2,
                ),
            )
        )

    return DamageTable(link_damage_rows=link_damage_rows)


def write_damage_table(rom: LocalRom, damage_table: DamageTable) -> None:
    for index, row in enumerate(iterable=damage_table.link_damage_rows):
        # Write Link's damage table.
        rom.write_snes_address(
            snes_address=rom.damage_table_snes + (index * 3),
            value=row.green_mail,
        )
        rom.write_snes_address(
            snes_address=rom.damage_table_snes + (index * 3) + 1,
            value=row.blue_mail,
        )
        rom.write_snes_address(
            snes_address=rom.damage_table_snes + (index * 3) + 2,
            value=row.red_mail,
        )
