from typing import List, Dict

from .LocalRom import LocalRom
from ..Model import SpriteBlocksetId, SpriteBlockset


def _read_sprite_blockset(rom: LocalRom, id: SpriteBlocksetId) -> List[SpriteBlockset]:
    set0 = rom.read_address(rom.sprite_blockset_address + (id * 4))
    set1 = rom.read_address(rom.sprite_blockset_address + (id * 4) + 1)
    set2 = rom.read_address(rom.sprite_blockset_address + (id * 4) + 2)
    set3 = rom.read_address(rom.sprite_blockset_address + (id * 4) + 3)
    return SpriteBlockset(
        id=id,
        set0=set0,
        set1=set1,
        set2=set2,
        set3=set3,
    )


def read_sprite_blocksets(rom: LocalRom) -> Dict[SpriteBlocksetId, SpriteBlockset]:
    return {id: _read_sprite_blockset(rom, id) for id in list(SpriteBlocksetId)}


def write_sprite_blocksets(
    rom: LocalRom,
    blocksets: Dict[SpriteBlocksetId, SpriteBlockset],
) -> None:
    for id, blockset in blocksets.items():
        rom.write_address(rom.sprite_blockset_address + (id * 4), blockset.set0)
        rom.write_address(rom.sprite_blockset_address + (id * 4) + 1, blockset.set1)
        rom.write_address(rom.sprite_blockset_address + (id * 4) + 2, blockset.set2)
        rom.write_address(rom.sprite_blockset_address + (id * 4) + 3, blockset.set3)
