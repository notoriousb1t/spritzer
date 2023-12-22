from typing import Callable, List, Dict

from ..Model import SpriteBlocksetId, SpriteBlockset

_sprite_blockset_address = 0x5B97


def _read_sprite_blockset(
    id: SpriteBlocksetId,
    read: Callable[[int], int],
) -> List[SpriteBlockset]:
    set0 = read(_sprite_blockset_address + (id * 4))
    set1 = read(_sprite_blockset_address + (id * 4) + 1)
    set2 = read(_sprite_blockset_address + (id * 4) + 2)
    set3 = read(_sprite_blockset_address + (id * 4) + 3)
    return SpriteBlockset(
        id=id,
        set0=set0,
        set1=set1,
        set2=set2,
        set3=set3,
    )


def read_sprite_blocksets(
    read: Callable[[int, int], int]
) -> Dict[SpriteBlocksetId, SpriteBlockset]:
    return {id: _read_sprite_blockset(id, read) for id in list(SpriteBlocksetId)}


def write_sprite_blocksets(
    blocksets: Dict[SpriteBlocksetId, SpriteBlockset],
    write: Callable[[int, int], None],
) -> None:
    for id, blockset in blocksets.items():
        write(_sprite_blockset_address + (id * 4), blockset.set0)
        write(_sprite_blockset_address + (id * 4) + 1, blockset.set1)
        write(_sprite_blockset_address + (id * 4) + 2, blockset.set2)
        write(_sprite_blockset_address + (id * 4) + 3, blockset.set3)
