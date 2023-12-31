from typing import List, Dict

from .LocalRom import LocalRom, resolve_address

from ..Model import (
    BlocksetId,
    UnderworldRoom,
    UnderworldRoomFloorId,
    UnderworldRoomId,
    UnderworldSprite,
    UnderworldRoomTag,
    PaletteId,
    SpriteId,
    SpritesetId,
)


_stop_marker = 0xFF
_overlord_offset = 0x100


def _peek_item(rom: LocalRom, sprite_address: int) -> SpriteId:
    if rom.read_address(sprite_address + 3) == _stop_marker:
        return None

    byte5 = rom.read_address(sprite_address + 5)
    if byte5 == SpriteId.xE4_KEY:
        return SpriteId.xE4_KEY
    if byte5 == SpriteId.xE5_BIG_KEY:
        return SpriteId.xE5_BIG_KEY
    return None


def _read_room_sprites(rom: LocalRom, base_address: int) -> List[UnderworldSprite]:
    index = 1  # byte 0 handles sprite ordering, there is no reason to modify this.
    dungeon_room_sprites = []
    remaining_max_bytes = 10000
    while True:
        # Read the sprite table for this Dungeon Room.
        sprite_address = base_address + index
        # Peek to look for 255: the stop character.
        # This happens when there are no more Sprites in the Dungeon Room.
        # More data appears to be after this marker, so this should remain
        # a fixed length.
        if rom.read_address(sprite_address) == _stop_marker:
            break
        if remaining_max_bytes < 1:
            raise "Maximum bytes exceeded. Aborting to prevent infinite loop"
        # Read from byte 0 lssyyyyy
        byte0 = rom.read_address(sprite_address)
        lower_layer = bool(byte0 & 0b1000_0000)
        aux0 = (byte0 & 0b0110_0000) >> 5
        y = byte0 & 0b0001_1111

        byte1 = rom.read_address(sprite_address + 1)
        x = byte1 & 0b0001_1111
        aux1 = (byte1 & 0b1110_0000) >> 5

        sprite_value = rom.read_address(sprite_address + 2)
        type = SpriteId(sprite_value + (_overlord_offset if aux1 == 0b111 else 0))
        item = _peek_item(rom, sprite_address)

        dungeon_room_sprites.append(
            UnderworldSprite(
                sprite_address,
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
    return dungeon_room_sprites


def _read_room(rom: LocalRom, id: UnderworldRoomId) -> UnderworldRoom:
    # Find the address of the Dungeon Room and read in graphics block and tags.
    header_address = resolve_address(
        [
            rom.room_header_bank,
            rom.read_snes_address(rom.room_header_pointers_snes + (id * 2) + 1),
            rom.read_snes_address(rom.room_header_pointers_snes + (id * 2)),
        ]
    )
    # Read in the graphics block which controls the spritesheets
    # and tags which declare behaviors.
    bg2_property = rom.read_address(header_address)
    palette_id = PaletteId(rom.read_address(header_address + 1))
    blockset_id = BlocksetId(rom.read_address(header_address + 2))
    spriteset_id = SpritesetId.from_room_value(rom.read_address(header_address + 3))
    bgmove = rom.read_address(header_address + 4)
    tag1 = UnderworldRoomTag(rom.read_address(header_address + 5))
    tag2 = UnderworldRoomTag(rom.read_address(header_address + 6))
    floor_upper = UnderworldRoomFloorId(rom.read_address(header_address + 7))
    floor_lower = UnderworldRoomFloorId(rom.read_address(header_address + 8))
    warp = UnderworldRoomId(rom.read_address(header_address + 9))
    stairs0 = UnderworldRoomId(rom.read_address(header_address + 10))
    stairs1 = UnderworldRoomId(rom.read_address(header_address + 11))
    stairs2 = UnderworldRoomId(rom.read_address(header_address + 12))
    stairs3 = UnderworldRoomId(rom.read_address(header_address + 13))

    sprite_ptr = (
        rom.read_snes_address(rom.room_sprite_pointers_snes + (id * 2)),
        rom.read_snes_address(rom.room_sprite_pointers_snes + (id * 2) + 1),
    )
    sprite_table_base_snes_address = resolve_address(
        [
            rom.underworld_sprite_bank,
            sprite_ptr[1],
            sprite_ptr[0],
        ]
    )

    dungeon_sprites = _read_room_sprites(rom, sprite_table_base_snes_address)

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
            dungeon_sprite._address,
            (0b1000_0000 if dungeon_sprite.lower_layer else 0)
            | ((dungeon_sprite.aux0 & 0b11) << 5)
            | dungeon_sprite.y,
        )
        rom.write_address(
            dungeon_sprite._address + 1,
            dungeon_sprite.x | ((dungeon_sprite.aux1 & 0b111) << 5),
        )
        rom.write_address(
            dungeon_sprite._address + 2,
            dungeon_sprite.sprite_id
            - (_overlord_offset if dungeon_sprite.sprite_id >= _overlord_offset else 0),
        )


def write_underworld_rooms(
    rom: LocalRom,
    dungeon_room_dict: Dict[UnderworldRoomId, UnderworldRoom],
) -> None:
    for room in dungeon_room_dict.values():
        rom.write_address(room.header_address, room.bg2_property)
        rom.write_address(room.header_address + 1, room.palette_id)
        rom.write_address(room.header_address + 2, room.blockset_id)
        rom.write_address(
            room.header_address + 3,
            room.spriteset_id.get_room_value(),
        )
        rom.write_address(room.header_address + 4, room.bgmove)
        rom.write_address(room.header_address + 5, room.tag1)
        rom.write_address(room.header_address + 6, room.tag2)
        rom.write_address(room.header_address + 7, room.floor_upper)
        rom.write_address(room.header_address + 8, room.floor_lower)
        rom.write_address(room.header_address + 9, room.warp)
        rom.write_address(room.header_address + 10, room.stairs0)
        rom.write_address(room.header_address + 11, room.stairs1)
        rom.write_address(room.header_address + 12, room.stairs2)
        rom.write_address(room.header_address + 13, room.stairs3)

        rom.write_snes_address(
            rom.room_sprite_pointers_snes + (room.id * 2),
            room.sprite_ptr[0],
        )
        rom.write_snes_address(
            rom.room_sprite_pointers_snes + (room.id * 2) + 1,
            room.sprite_ptr[1],
        )

        _write_dungeon_sprites(rom, room)
