from typing import Dict

from ..Model.SpritesetId import SpritesetId

from ..Model.OverworldSprite import OverworldSprite

from ..Model.OverworldAreaId import OverworldAreaId

from .LocalRom import LocalRom, compute_snes_address

from ..Model import (
    OverworldArea,
    SpriteId,
)

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


def _load_area(rom: LocalRom, id: OverworldAreaId) -> OverworldArea:
    """Reads an Area from the ROM and returns it as a data class."""
    # Resolve the address of this Dungeon RoomArea and read the graphics block into memory.
    gfx = SpritesetId(rom.read_address(_resolve_gfx_address(id)))

    # Find the base address of Overworld Sprites in this Overworld Area.
    sprite_table_base_address = compute_snes_address(
        [
            rom.overworld_sprite_bank,
            rom.read_snes_address(rom.area_sprite_pointers_snes + (id * 2) + 1),
            rom.read_snes_address(rom.area_sprite_pointers_snes + (id * 2)),
        ]
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
        if rom.read_snes_address(address) == _stop_marker:
            break
        if remaining_max_bytes == 0:
            raise "Maximum bytes exceeded. Aborting to prevent infinite loop"

        overworld_sprites.append(
            OverworldSprite(
                address,
                y=rom.read_snes_address(address),
                x=rom.read_snes_address(address + 1),
                id=SpriteId(rom.read_snes_address(address + 2)),
            )
        )
        index += 3
        remaining_max_bytes -= 1
    return OverworldArea(id, gfx, overworld_sprites)


def read_overworld_areas(rom: LocalRom) -> Dict[OverworldAreaId, OverworldArea]:
    """Returns AreaBlocks for each Area based on the ROM."""
    return {id: _load_area(rom, id) for id in list(OverworldAreaId)}


def write_overworld_areas(
    rom: LocalRom,
    overworld_area_dict: Dict[OverworldAreaId, OverworldArea],
) -> None:
    """Writes AreaBlocks back to the ROM data."""

    for id, overworld_area in overworld_area_dict.items():
        # Write the new graphics block back to the Overworld Area address.
        rom.write_address(_resolve_gfx_address(id), overworld_area.spriteset_id)

        # Rewrite Overworld Sprites back into the same spots.
        for overworld_sprite in overworld_area.overworld_sprites:
            rom.write_snes_address(overworld_sprite._address, overworld_sprite.y)
            rom.write_snes_address(overworld_sprite._address + 1, overworld_sprite.x)
            rom.write_snes_address(overworld_sprite._address + 2, overworld_sprite.id)
