from enum import IntEnum
from typing import Dict, List

from ..Model import (
    SpritesetId,
    OverworldId,
    OverworldSprite,
    OverworldAreaId,
    OverworldAreaRoom,
)

from .LocalRom import LocalRom, compute_snes_address, pc_address_to_snes_address

from ..Model import (
    OverworldArea,
    SpriteId,
)

_stop_marker = 0xFF


def _get_sprite_graphics_address(
    rom: LocalRom,
    id: OverworldAreaId,
    overworld_id: OverworldId,
) -> int:
    if id == OverworldAreaId.x40_MASTER_SWORD_UNDER_BRIDGE:
        return rom.area_special_graphics_snes
    if id == OverworldAreaId.x41_ZORAS_DOMAIN_POST:
        return rom.area_special_graphics_snes + 1
    return rom.area_graphics_snes + id + (overworld_id * 0x40)


def _get_palette_address(
    rom: LocalRom,
    id: OverworldAreaId,
    overworld_id: OverworldId,
) -> int:
    if id == OverworldAreaId.x40_MASTER_SWORD_UNDER_BRIDGE:
        return rom.area_special_palette_snes
    if id == OverworldAreaId.x41_ZORAS_DOMAIN_POST:
        return rom.area_special_palette_snes + 1
    return rom.area_graphics_snes + id + ((overworld_id + 4) * 0x40)


def _get_sprite_pointer_address(
    rom: LocalRom,
    id: OverworldAreaId,
    overworld_id: OverworldId,
) -> int:
    base_address: int
    if overworld_id == OverworldId.DARK_WORLD:
        base_address = rom.area_sprite_pointers_darkworld_v2_snes
    elif overworld_id == OverworldId.LIGHT_WORLD_V1:
        base_address = rom.area_sprite_pointers_lightworld_v1_snes
    else:
        base_address = rom.area_sprite_pointers_lightworld_v2_snes
    if id >= 0x40:
        base_address += 0x40
    return base_address + (id * 2)


def _load_sprites(rom: LocalRom, sprite_address: int) -> List[OverworldSprite]:
    # Find the base address of Overworld Sprites in this Overworld Area.
    sprite_table_base_address = compute_snes_address(
        [
            rom.overworld_sprite_bank,
            rom.read_snes_address(sprite_address + 1),
            rom.read_snes_address(sprite_address),
        ]
    )

    index = 0
    sprites = []
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

        y = rom.read_snes_address(address)
        x = rom.read_snes_address(address + 1)
        sprite_id = rom.read_snes_address(address + 2)

        sprites.append(OverworldSprite(address, y=y, x=x, id=SpriteId(sprite_id)))
        index += 3
        remaining_max_bytes -= 1

    return sprites


def _load_room_for_area(
    rom: LocalRom,
    id: OverworldAreaId,
    overworld_id: OverworldId,
) -> OverworldAreaRoom:
    """Reads an Area from the ROM and returns it as a data class."""
    # Resolve the sprite graphics and sprite palette id.
    spriteset_id = SpritesetId(
        rom.read_snes_address(
            _get_sprite_graphics_address(
                rom=rom,
                id=id,
                overworld_id=overworld_id,
            )
        )
    )
    sprite_palette_id = rom.read_snes_address(
        _get_palette_address(
            rom=rom,
            id=id,
            overworld_id=overworld_id,
        )
    )
    sprites = _load_sprites(
        rom,
        sprite_address=_get_sprite_pointer_address(
            rom=rom,
            id=id,
            overworld_id=overworld_id,
        ),
    )

    return OverworldAreaRoom(
        overworld_id=overworld_id,
        spriteset_id=spriteset_id,
        sprite_palette_id=sprite_palette_id,
        sprites=sprites,
    )


def _load_area(rom: LocalRom, id: OverworldAreaId) -> OverworldArea:
    if id >= 0x40:
        # Load Zoras domain, master sword grove, and under bridge
        # from post aga pointer. These are after the normal rooms which have
        # light and dark world equivalents.
        light_world_v2 = _load_room_for_area(
            rom=rom, id=id, overworld_id=OverworldId.LIGHT_WORLD_V2
        )

        return OverworldArea(id=id, light_world_v2=light_world_v2)

    light_world_v1 = _load_room_for_area(
        rom=rom,
        id=id,
        overworld_id=OverworldId.LIGHT_WORLD_V1,
    )
    light_world_v2 = _load_room_for_area(
        rom=rom,
        id=id,
        overworld_id=OverworldId.LIGHT_WORLD_V2,
    )
    dark_world = _load_room_for_area(
        rom=rom,
        id=id,
        overworld_id=OverworldId.DARK_WORLD,
    )

    return OverworldArea(
        id=id,
        light_world_v1=light_world_v1,
        light_world_v2=light_world_v2,
        dark_world=dark_world,
    )


def read_overworld_areas(rom: LocalRom) -> Dict[OverworldAreaId, OverworldArea]:
    """Returns AreaBlocks for each Area based on the ROM."""
    return {id: _load_area(rom=rom, id=id) for id in list(OverworldAreaId)}


def write_overworld_areas(
    rom: LocalRom,
    overworld_areas: Dict[OverworldAreaId, OverworldArea],
) -> None:
    """Writes AreaBlocks back to the ROM data."""

    for area_id, overworld_area in overworld_areas.items():
        for version in overworld_area.versions:
            # Write the new graphics block back to the Overworld Area address.
            rom.write_snes_address(
                _get_sprite_graphics_address(
                    rom=rom,
                    id=area_id,
                    overworld_id=version.overworld_id,
                ),
                version.spriteset_id,
            )
            rom.write_snes_address(
                _get_palette_address(
                    rom=rom,
                    id=area_id,
                    overworld_id=version.overworld_id,
                ),
                version.sprite_palette_id,
            )

            # Rewrite Overworld Sprites back into the same spots.
            for sprite in version.sprites:
                rom.write_snes_address(sprite._address, sprite.y)
                rom.write_snes_address(sprite._address + 1, sprite.x)
                rom.write_snes_address(sprite._address + 2, sprite.id)
