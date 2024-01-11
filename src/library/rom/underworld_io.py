from typing import List, Dict, Union

from library.model.blockset_id import BlocksetId
from library.model.palette_id import PaletteId
from library.model.sprite_id import SpriteId
from library.model.spriteset_id import SpritesetId
from library.model.underworld_room import UnderworldRoom
from library.model.underworld_room_floor_id import UnderworldRoomFloorId
from library.model.underworld_room_id import UnderworldRoomId
from library.model.underworld_room_tag import UnderworldRoomTag
from library.model.underworld_sprite import UnderworldSprite
from library.rom.local_rom import LocalRom, resolve_address


_stop_marker = 0xFF
_overlord_offset = 0x100


def _peek_item(rom: LocalRom, sprite_address: int) -> Union[SpriteId, None]:
    if rom.read_address(address=sprite_address + 3) == _stop_marker:
        return None

    byte5 = rom.read_address(address=sprite_address + 5)
    if byte5 == SpriteId.xE4_KEY:
        return SpriteId.xE4_KEY
    if byte5 == SpriteId.xE5_BIG_KEY:
        return SpriteId.xE5_BIG_KEY
    return None


def _read_room_sprites(rom: LocalRom, base_address: int) -> List[UnderworldSprite]:
    index = 1  # byte 0 handles sprite ordering, there is no reason to modify this.
    underworld_sprites: List[UnderworldSprite] = []
    remaining_max_bytes = 10000
    while True:
        # Read the sprite table for this Dungeon Room.
        sprite_address: int = base_address + index
        # Peek to look for 255: the stop character.
        # This happens when there are no more Sprites in the Dungeon Room.
        # More data appears to be after this marker, so this should remain
        # a fixed length.
        if rom.read_address(address=sprite_address) == _stop_marker:
            break
        if remaining_max_bytes < 1:
            raise "Maximum bytes exceeded. Aborting to prevent infinite loop" # type: ignore
        # Read from byte 0 lssyyyyy
        byte0: int = rom.read_address(address=sprite_address)
        lower_layer = bool(byte0 & 0b1000_0000)
        aux0: int = (byte0 & 0b0110_0000) >> 5
        y: int = byte0 & 0b0001_1111

        byte1: int = rom.read_address(address=sprite_address + 1)
        x: int = byte1 & 0b0001_1111
        aux1: int = (byte1 & 0b1110_0000) >> 5

        sprite_value: int = rom.read_address(address=sprite_address + 2)
        type = SpriteId(value=sprite_value + (_overlord_offset if aux1 == 0b111 else 0))
        item: Union[SpriteId, None] = _peek_item(rom=rom, sprite_address=sprite_address)

        underworld_sprites.append(
            UnderworldSprite(
                address=sprite_address,
                sprite_id=type,
                y=y,
                x=x,
                lower_layer=lower_layer,
                aux0=aux0,
                aux1=aux1,
                item=item,
            )
        )
        index += 3
        remaining_max_bytes -= 3
    return underworld_sprites


def _read_room(rom: LocalRom, id: UnderworldRoomId) -> UnderworldRoom:
    # Find the address of the Dungeon Room and read in graphics block and tags.
    header_address: int = resolve_address(
        byte_values=[
            rom.room_header_bank,
            rom.read_snes_address(snes_address=rom.room_header_pointers_snes + (id * 2) + 1),
            rom.read_snes_address(snes_address=rom.room_header_pointers_snes + (id * 2)),
        ]
    )
    # Read in the graphics block which controls the spritesheets
    # and tags which declare behaviors.
    bg2_property: int = rom.read_address(address=header_address)
    palette_id = PaletteId(value=rom.read_address(address=header_address + 1))
    blockset_id = BlocksetId(value=rom.read_address(address=header_address + 2))
    spriteset_id: SpritesetId = SpritesetId.from_room_value(value=rom.read_address(address=header_address + 3))
    bgmove: int = rom.read_address(address=header_address + 4)
    tag1 = UnderworldRoomTag(value=rom.read_address(address=header_address + 5))
    tag2 = UnderworldRoomTag(value=rom.read_address(address=header_address + 6))
    floor_upper = UnderworldRoomFloorId(value=rom.read_address(address=header_address + 7))
    floor_lower = UnderworldRoomFloorId(value=rom.read_address(address=header_address + 8))
    warp = UnderworldRoomId(value=rom.read_address(address=header_address + 9))
    stairs0 = UnderworldRoomId(value=rom.read_address(address=header_address + 10))
    stairs1 = UnderworldRoomId(value=rom.read_address(address=header_address + 11))
    stairs2 = UnderworldRoomId(value=rom.read_address(address=header_address + 12))
    stairs3 = UnderworldRoomId(value=rom.read_address(address=header_address + 13))

    sprite_ptr: tuple[int, int] = (
        rom.read_snes_address(snes_address=rom.room_sprite_pointers_snes + (id * 2)),
        rom.read_snes_address(snes_address=rom.room_sprite_pointers_snes + (id * 2) + 1),
    )
    sprite_table_base_snes_address: int = resolve_address(
        byte_values=[
            rom.underworld_sprite_bank,
            sprite_ptr[1],
            sprite_ptr[0],
        ]
    )

    dungeon_sprites: List[UnderworldSprite] = _read_room_sprites(rom=rom, base_address=sprite_table_base_snes_address)

    return UnderworldRoom(
        header_address=header_address,
        id=id,
        bg2_property=bg2_property,
        palette_id=palette_id,
        blockset_id=blockset_id,
        spriteset_id=spriteset_id,
        bgmove=bgmove,
        tag1=tag1,
        tag2=tag2,
        floor_upper=floor_upper,
        floor_lower=floor_lower,
        warp=warp,
        stairs0=stairs0,
        stairs1=stairs1,
        stairs2=stairs2,
        stairs3=stairs3,
        sprite_ptr=sprite_ptr,
        sprites=dungeon_sprites,
    )


def read_underworld_rooms(rom: LocalRom) -> Dict[UnderworldRoomId, UnderworldRoom]:
    return {id: _read_room(rom, id) for id in list(UnderworldRoomId)}


def _write_dungeon_sprites(rom: LocalRom, room: UnderworldRoom) -> None:
    # Rewrite new Dungeon Sprites.
    for dungeon_sprite in room.sprites:
        rom.write_address(
            address=dungeon_sprite.address,
            value=(0b1000_0000 if dungeon_sprite.lower_layer else 0)
            | ((dungeon_sprite.aux0 & 0b11) << 5)
            | dungeon_sprite.y,
        )
        rom.write_address(
            address=dungeon_sprite.address + 1,
            value=dungeon_sprite.x | ((dungeon_sprite.aux1 & 0b111) << 5),
        )
        rom.write_address(
            address=dungeon_sprite.address + 2,
            value=dungeon_sprite.sprite_id
            - (_overlord_offset if dungeon_sprite.sprite_id >= _overlord_offset else 0),
        )


def write_underworld_rooms(
    rom: LocalRom,
    dungeon_room_dict: Dict[UnderworldRoomId, UnderworldRoom],
) -> None:
    for room in dungeon_room_dict.values():
        rom.write_address(address=room.header_address, value=room.bg2_property)
        rom.write_address(address=room.header_address + 1, value=room.palette_id)
        rom.write_address(address=room.header_address + 2, value=room.blockset_id)
        rom.write_address(
            address=room.header_address + 3,
            value=room.spriteset_id.get_room_value(),
        )
        rom.write_address(address=room.header_address + 4, value=room.bgmove)
        rom.write_address(address=room.header_address + 5, value=room.tag1)
        rom.write_address(address=room.header_address + 6, value=room.tag2)
        rom.write_address(address=room.header_address + 7, value=room.floor_upper)
        rom.write_address(address=room.header_address + 8, value=room.floor_lower)
        rom.write_address(address=room.header_address + 9, value=room.warp)
        rom.write_address(address=room.header_address + 10, value=room.stairs0)
        rom.write_address(address=room.header_address + 11, value=room.stairs1)
        rom.write_address(address=room.header_address + 12, value=room.stairs2)
        rom.write_address(address=room.header_address + 13, value=room.stairs3)

        rom.write_snes_address(
            snes_address=rom.room_sprite_pointers_snes + (room.id * 2),
            value=room.sprite_ptr[0],
        )
        rom.write_snes_address(
            snes_address=rom.room_sprite_pointers_snes + (room.id * 2) + 1,
            value=room.sprite_ptr[1],
        )

        _write_dungeon_sprites(rom, room)
