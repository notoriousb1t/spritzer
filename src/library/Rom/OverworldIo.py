from typing import Dict, List

from ..Model import (
    SpritesetId,
    OverworldSprite,
    OverworldAreaId,
    OverworldConfiguration,
)

from .LocalRom import LocalRom, compute_snes_address, pc_address_to_snes_address

from ..Model import (
    OverworldArea,
    SpriteId,
)

_stop_marker = 0xFF
_light_world_v1_index = 1
_light_world_v2_index = 2
_darkworld_index = 3

#
# if (
#     id == OverworldAreaId.x80_MASTER_SWORD_GLADE_UNDER_BRIDGE
#     or id == OverworldAreaId.x81_ZORAS_DOMAIN
# ):
#     return 0x016576 + id - 0x80
# if (
#     id == OverworldAreaId.x100_MISERY_MIRE_DW_POST_AGA
#     or id == OverworldAreaId.x111_ZORAS_DOMAIN_POST_AGA
# ):


def _load_sprites(
    rom: LocalRom, id: OverworldAreaId, sprite_address: int
) -> List[OverworldSprite]:
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

        sprite_id = rom.read_snes_address(address + 2)
        sprites.append(
            OverworldSprite(
                address,
                y=rom.read_snes_address(address),
                x=rom.read_snes_address(address + 1),
                id=SpriteId(sprite_id),
            )
        )
        index += 3
        remaining_max_bytes -= 1

    return sprites


def _load_configuration(
    rom: LocalRom,
    id: OverworldAreaId,
    config_index: int,
    sprite_address: int,
) -> OverworldConfiguration:
    """Reads an Area from the ROM and returns it as a data class."""
    # Resolve the sprite graphics and sprite palette id.
    spriteset_id = SpritesetId(
        rom.read_snes_address(rom.area_graphics_snes + id + (config_index * 0x40))
    )
    sprite_palette_id = rom.read_snes_address(
        rom.area_graphics_snes + id + ((config_index + 4) * 0x40)
    )
    sprites = _load_sprites(
        rom,
        id=id,
        sprite_address=sprite_address + (id * 2) + (0x40 if id >= 0x40 else 0),
    )

    return OverworldConfiguration(
        spriteset_id=spriteset_id,
        sprite_palette_id=sprite_palette_id,
        sprites=sprites,
    )


def _load_area(rom: LocalRom, id: OverworldAreaId) -> OverworldArea:
    if id >= 0x40:
        # Load Zoras domain, master sword grove, and under bridge
        # from post aga pointer. These are after the normal rooms which have
        # light and dark world equivalents.
        return OverworldArea(
            id=id,
            light_world_v2=_load_configuration(
                rom,
                id=id,
                config_index=_light_world_v2_index,
                sprite_address=rom.area_sprite_pointers_lightworld_v2_snes,
            ),
        )

    light_world_v1 = _load_configuration(
        rom,
        id=id,
        config_index=_light_world_v1_index,
        sprite_address=rom.area_sprite_pointers_lightworld_v1_snes,
    )
    light_world_v2 = _load_configuration(
        rom,
        id=id,
        config_index=_light_world_v2_index,
        sprite_address=rom.area_sprite_pointers_lightworld_v2_snes,
    )
    dark_world = _load_configuration(
        rom,
        id=id,
        config_index=_darkworld_index,
        sprite_address=rom.area_sprite_pointers_darkworld_v2_snes,
    )

    return OverworldArea(
        id,
        light_world_v1=light_world_v1,
        light_world_v2=light_world_v2,
        dark_world=dark_world,
    )


def read_overworld_areas(rom: LocalRom) -> Dict[OverworldAreaId, OverworldArea]:
    """Returns AreaBlocks for each Area based on the ROM."""
    return {id: _load_area(rom, id) for id in list(OverworldAreaId)}


def write_overworld_areas(
    rom: LocalRom,
    overworld_areas: Dict[OverworldAreaId, OverworldArea],
) -> None:
    """Writes AreaBlocks back to the ROM data."""

    for area_id, overworld_area in overworld_areas.items():
        for config_id, configuration in enumerate(overworld_area.configurations):
            # Write the new graphics block back to the Overworld Area address.
            rom.write_address(
                rom.area_graphics_snes + area_id + (config_id * 0x40),
                configuration.spriteset_id,
            )
            rom.write_address(
                rom.area_graphics_snes + area_id + ((config_id + 4) * 0x40),
                configuration.sprite_palette_id,
            )

            # Rewrite Overworld Sprites back into the same spots.
            for sprite in configuration.sprites:
                rom.write_snes_address(sprite._address, sprite.y)
                rom.write_snes_address(sprite._address + 1, sprite.x)
                rom.write_snes_address(sprite._address + 2, sprite.id)
