from .LocalRom import LocalRom
from ..Model import DamageRow, DamageTable


def read_damage_table(rom: LocalRom) -> DamageTable:
    link_damage_rows = list()
    for index in range(0, 10):
        # Read Link's damage table.
        link_damage_rows.append(
            DamageRow(
                green_mail=rom.read_snes_address(
                    rom.damage_table_snes_address + (index * 3),
                ),
                blue_mail=rom.read_snes_address(
                    rom.damage_table_snes_address + (index * 3) + 1,
                ),
                red_mail=rom.read_snes_address(
                    rom.damage_table_snes_address + (index * 3) + 2,
                ),
            )
        )

    return DamageTable(link_damage_rows=link_damage_rows)


def write_damage_table(rom: LocalRom, damage_table: DamageTable) -> None:
    for index, row in enumerate(damage_table.link_damage_rows):
        # Write Link's damage table.
        rom.write_snes_address(
            rom.damage_table_snes_address + (index * 3),
            row.green_mail,
        )
        rom.write_snes_address(
            rom.damage_table_snes_address + (index * 3) + 1,
            row.blue_mail,
        )
        rom.write_snes_address(
            rom.damage_table_snes_address + (index * 3) + 2,
            row.red_mail,
        )
