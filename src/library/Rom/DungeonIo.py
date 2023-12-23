from typing import Callable, List, Dict

from .LocalRom import LocalRom, resolve_address

from ..Model import (
    DungeonRoomId,
    DungeonRoom,
    DungeonSprite,
    DungeonRoomTilesetId,
    DungeonRoomTilesetId,
    DungeonTag,
    SpriteBlocksetId,
    SpriteId,
)


_stop_marker = 0xFF
_remove_subtype = 0b10011111
_remove_overlord = 0b00011111
_has_subtype = 0b01100000
_has_overlord = 0b11100000
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


def _load_room_entities(rom: LocalRom, base_address: int) -> List[DungeonSprite]:
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

        byte0 = rom.read_address(sprite_address)
        byte1 = rom.read_address(sprite_address + 1)
        y = byte0 & _remove_subtype
        x = byte1 & _remove_overlord
        is_subtype = byte0 & _has_subtype == _has_subtype
        is_overlord = byte1 & _has_overlord == _has_overlord
        sprite_value = rom.read_address(sprite_address + 2)
        type = SpriteId(sprite_value + (_overlord_offset if is_overlord else 0))
        item = _peek_item(rom, sprite_address)

        dungeon_room_sprites.append(
            DungeonSprite(
                sprite_address,
                sprite_id=type,
                y=y,
                x=x,
                is_overlord=is_overlord,
                is_subtype=is_subtype,
                item=item,
            )
        )
        index += 3
        remaining_max_bytes -= 3
    return dungeon_room_sprites


def _read_room(rom: LocalRom, id: DungeonRoomId) -> DungeonRoom:
    # Find the address of the Dungeon Room and read in graphics block and tags.
    header_address = resolve_address(
        rom.read_address(rom.dungeon_room_pointer_header_address + (id * 2)),
        rom.read_address(rom.dungeon_room_pointer_header_address + (id * 2) + 1),
        rom.dungeon_room_bank,
    )
    # Read in the graphics block which controls the spritesheets
    # and tags which declare behaviors.
    lights_out_effect = bool(rom.read_address(header_address) & 0b1 == 0b1)
    palette_id = rom.read_address(header_address + 1)
    tileset_id = DungeonRoomTilesetId(rom.read_address(header_address + 2))
    blockset_id = SpriteBlocksetId.from_room_value(rom.read_address(header_address + 3))
    effect = rom.read_address(header_address + 4)
    tag1 = DungeonTag(rom.read_address(header_address + 5))
    tag2 = DungeonTag(rom.read_address(header_address + 6))

    dungeon_sprite_pointer_address = rom.dungeon_sprite_ptr_table_address + (id * 2)

    sprite_ptr = (
        rom.read_address(dungeon_sprite_pointer_address),
        rom.read_address(dungeon_sprite_pointer_address + 1),
    )
    sprite_table_base_snes_address = resolve_address(
        sprite_ptr[0], sprite_ptr[1], rom.dungeon_sprite_bank
    )

    dungeon_sprites = _load_room_entities(rom, sprite_table_base_snes_address)

    return DungeonRoom(
        header_address=header_address,
        id=id,
        lights_out_effect=lights_out_effect,
        palette_id=palette_id,
        tileset_id=tileset_id,
        blockset_id=blockset_id,
        effect=effect,
        tag1=tag1,
        tag2=tag2,
        sprite_ptr=sprite_ptr,
        dungeon_sprites=dungeon_sprites,
    )


def read_dungeon_rooms(rom: LocalRom) -> Dict[DungeonRoomId, DungeonRoom]:
    return {id: _read_room(rom, id) for id in list(DungeonRoomId)}


def write_dungeon_rooms(
    rom: LocalRom,
    dungeon_room_dict: Dict[DungeonRoomId, DungeonRoom],
) -> None:
    for room in dungeon_room_dict.values():
        rom.write_address(room.header_address + 1, room.palette_id)
        rom.write_address(room.header_address + 2, room.tileset_id)
        rom.write_address(
            room.header_address + 3,
            room.blockset_id.get_room_value(),
        )
        rom.write_address(room.header_address + 4, room.effect)
        rom.write_address(room.header_address + 5, room.tag1)
        rom.write_address(room.header_address + 6, room.tag2)

        sprite_ptr_address = rom.dungeon_sprite_ptr_table_address + (room.id * 2)
        rom.write_address(sprite_ptr_address, room.sprite_ptr[0]),
        rom.write_address(sprite_ptr_address + 1, room.sprite_ptr[1])

        # Rewrite new Dungeon Sprites.
        for dungeon_sprite in room.dungeon_sprites:
            rom.write_address(
                dungeon_sprite._address,
                dungeon_sprite.y | _has_subtype
                if dungeon_sprite.sprite_id == SpriteId.xE4_KEY
                else dungeon_sprite.y,
            )
            rom.write_address(
                dungeon_sprite._address + 1,
                dungeon_sprite.x | _has_overlord
                if dungeon_sprite.sprite_id >= _overlord_offset
                else dungeon_sprite.x,
            )
            rom.write_address(
                dungeon_sprite._address + 2,
                dungeon_sprite.sprite_id
                - (
                    _overlord_offset
                    if dungeon_sprite.sprite_id >= _overlord_offset
                    else 0
                ),
            )
