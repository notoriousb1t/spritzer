from typing import Dict
from library.model.spriteset import Spriteset
from library.model.spriteset_id import SpritesetId
from library.model.sprite_sheet_id import SpriteSheetId
from library.rom.local_rom import LocalRom


def _read_spriteset(rom: LocalRom, id: SpritesetId) -> Spriteset:
    sheet0: int = rom.read_snes_address(
        snes_address=rom.sprite_blockset_snes + (id * 4) + 0
    )
    sheet1: int = rom.read_snes_address(
        snes_address=rom.sprite_blockset_snes + (id * 4) + 1
    )
    sheet2: int = rom.read_snes_address(
        snes_address=rom.sprite_blockset_snes + (id * 4) + 2
    )
    sheet3: int = rom.read_snes_address(
        snes_address=rom.sprite_blockset_snes + (id * 4) + 3
    )
    return Spriteset(
        id=id,
        sheet0=SpriteSheetId(value=sheet0),
        sheet1=SpriteSheetId(value=sheet1),
        sheet2=SpriteSheetId(value=sheet2),
        sheet3=SpriteSheetId(value=sheet3),
    )


def read_spritesets(rom: LocalRom) -> Dict[SpritesetId, Spriteset]:
    return {id: _read_spriteset(rom=rom, id=id) for id in list(SpritesetId)}


def write_spritesets(
    rom: LocalRom,
    blocksets: Dict[SpritesetId, Spriteset],
) -> None:
    for id, blockset in blocksets.items():
        rom.write_snes_address(
            snes_address=rom.sprite_blockset_snes + (id * 4) + 0, value=blockset.sheet0
        )
        rom.write_snes_address(
            snes_address=rom.sprite_blockset_snes + (id * 4) + 1, value=blockset.sheet1
        )
        rom.write_snes_address(
            snes_address=rom.sprite_blockset_snes + (id * 4) + 2, value=blockset.sheet2
        )
        rom.write_snes_address(
            snes_address=rom.sprite_blockset_snes + (id * 4) + 3, value=blockset.sheet3
        )
