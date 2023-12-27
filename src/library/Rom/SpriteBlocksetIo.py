from typing import List, Dict

from ..Model.SpritesetId import SpritesetId

from .LocalRom import LocalRom
from ..Model import Spriteset


def _read_sprite_blockset(rom: LocalRom, id: SpritesetId) -> List[Spriteset]:
    set0 = rom.read_address(rom.sprite_blockset_address + (id * 4))
    set1 = rom.read_address(rom.sprite_blockset_address + (id * 4) + 1)
    set2 = rom.read_address(rom.sprite_blockset_address + (id * 4) + 2)
    set3 = rom.read_address(rom.sprite_blockset_address + (id * 4) + 3)
    return Spriteset(
        id=id,
        sheet0=set0,
        sheet1=set1,
        sheet2=set2,
        sheet3=set3,
    )


def read_spritesets(rom: LocalRom) -> Dict[SpritesetId, Spriteset]:
    return {id: _read_sprite_blockset(rom, id) for id in list(SpritesetId)}


def write_sprite_blocksets(
    rom: LocalRom,
    blocksets: Dict[SpritesetId, Spriteset],
) -> None:
    for id, blockset in blocksets.items():
        rom.write_address(rom.sprite_blockset_address + (id * 4), blockset.sheet0)
        rom.write_address(rom.sprite_blockset_address + (id * 4) + 1, blockset.sheet1)
        rom.write_address(rom.sprite_blockset_address + (id * 4) + 2, blockset.sheet2)
        rom.write_address(rom.sprite_blockset_address + (id * 4) + 3, blockset.sheet3)
