from typing import Callable, Dict

from ..Model import (
    OverworldAreaId,
    OverworldArea,
    OverworldSprite,
    SpriteBlocksetId,
    SpriteId,
)
from .Utils import resolve_address

_bank_address = 0x09
_sprite_pointer_table_address = 0x04C901
_stop_marker = 0xFF


def _resolve_gfx_address(id: OverworldAreaId) -> int:
    """Finds the address that holds the graphics block id."""

    if (
        id == OverworldAreaId.x80_MASTER_SWORD_GLADE_UNDER_BRIDGE
        or id == OverworldAreaId.x81_ZORAS_DOMAIN
    ):
        return 0x016576 + id - 0x80
    if (
        id == OverworldAreaId.x100_MISERY_MIRE_DW_POST_AGA
        or id == OverworldAreaId.x111_ZORAS_DOMAIN_POST_AGA
    ):
        return 0x016576 + id - 0x110
    if id >= 0x40 and id < 0x80:
        return 0x007A81 + id + 0x40
    if id >= 0x90 and id < 0x110:
        return 0x007A81 + id - 0x50
    return 0x007A81 + id


def _load_area(
    id: OverworldAreaId, read_address: Callable[[int], int]
) -> OverworldArea:
    """Reads an Area from the ROM and returns it as a data class."""
    # Resolve the address of this Dungeon RoomArea and read the graphics block into memory.
    sprite_blockset_address = _resolve_gfx_address(id)
    gfx = SpriteBlocksetId(read_address(sprite_blockset_address))

    # Find the base address of Overworld Sprites in this Overworld Area.
    sprite_table_base_address = resolve_address(
        read_address(_sprite_pointer_table_address + (id * 2)),
        read_address(_sprite_pointer_table_address + (id * 2) + 1),
        _bank_address,
    )

    index = 0
    overworld_sprites = []
    remaining_max_bytes = 10000
    while True:
        # Read the sprite table for this Overworld Area.
        address = sprite_table_base_address + index
        # Peek to look for 255: the stop character.
        # This happens when there are no more Overworld Sprites in the Overworld Area.
        # More data appears to be after this marker, so this should remain
        # a fixed length.
        if read_address(address) == _stop_marker:
            break
        if remaining_max_bytes == 0:
            raise "Maximum bytes exceeded. Aborting to prevent infinite loop"

        overworld_sprites.append(
            OverworldSprite(
                address,
                y=read_address(address),
                x=read_address(address + 1),
                id=SpriteId(read_address(address + 2)),
            )
        )
        index += 3
        remaining_max_bytes -= 1
    return OverworldArea(id, gfx, overworld_sprites, sprite_blockset_address)


def write_overworld_areas(
    read_address: Callable[[int], int]
) -> Dict[OverworldAreaId, OverworldArea]:
    """Returns AreaBlocks for each Area based on the ROM."""
    return {id: _load_area(id, read_address) for id in list(OverworldAreaId)}


def read_overworld_areas(
    overworld_area_dict: Dict[OverworldAreaId, OverworldArea],
    write_address: Callable[[int, int], None],
) -> None:
    """Writes AreaBlocks back to the ROM data."""

    for id, overworld_area in overworld_area_dict.items():
        # Write the new graphics block back to the Overworld Area address.
        write_address(_resolve_gfx_address(id), overworld_area.blockset_id)

        # Rewrite Overworld Sprites back into the same spots.
        for overworld_sprite in overworld_area.overworld_sprites:
            write_address(overworld_sprite._address, overworld_sprite.y)
            write_address(overworld_sprite._address + 1, overworld_sprite.x)
            write_address(overworld_sprite._address + 2, overworld_sprite.id)
