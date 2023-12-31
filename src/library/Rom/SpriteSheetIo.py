from typing import List, Dict

from ..Model import SpritesetId, Spriteset, SpriteSheetId
from .LocalRom import LocalRom


def _read_spriteset(rom: LocalRom, id: SpritesetId) -> List[Spriteset]:
    set0 = rom.read_snes_address(rom.sprite_blockset_snes + (id * 4) + 0)
    set1 = rom.read_snes_address(rom.sprite_blockset_snes + (id * 4) + 1)
    set2 = rom.read_snes_address(rom.sprite_blockset_snes + (id * 4) + 2)
    set3 = rom.read_snes_address(rom.sprite_blockset_snes + (id * 4) + 3)
    return Spriteset(
        id=id,
        sheet0=SpriteSheetId(set0),
        sheet1=SpriteSheetId(set1),
        sheet2=SpriteSheetId(set2),
        sheet3=SpriteSheetId(set3),
    )


def read_spritesets(rom: LocalRom) -> Dict[SpritesetId, Spriteset]:
    return {id: _read_spriteset(rom, id) for id in list(SpritesetId)}


def write_spritesets(
    rom: LocalRom,
    blocksets: Dict[SpritesetId, Spriteset],
) -> None:
    for id, blockset in blocksets.items():
        rom.write_snes_address(rom.sprite_blockset_snes + (id * 4) + 0, blockset.sheet0)
        rom.write_snes_address(rom.sprite_blockset_snes + (id * 4) + 1, blockset.sheet1)
        rom.write_snes_address(rom.sprite_blockset_snes + (id * 4) + 2, blockset.sheet2)
        rom.write_snes_address(rom.sprite_blockset_snes + (id * 4) + 3, blockset.sheet3)
