from typing import Callable, List, Dict

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
from .Utils import resolve_address


_room_bank_address = 0x04
_room_sprite_bank_address = 0x09
_room_pointer_header_address = 0x271E2
_sprite_pointer_table_address = 0x4D62E
_stop_marker = 0xFF

_remove_subtype = 0b10011111
_remove_overlord = 0b00011111
_has_subtype = 0b01100000
_has_overlord = 0b11100000
_overlord_offset = 0x100


def _peek_item(
    sprite_address: int,
    read_address: Callable[[int], int],
) -> SpriteId:
    if read_address(sprite_address + 3) == _stop_marker:
        return None

    byte5 = read_address(sprite_address + 5)
    if byte5 == SpriteId.xE4_KEY:
        return SpriteId.xE4_KEY
    if byte5 == SpriteId.xE5_BIG_KEY:
        return SpriteId.xE5_BIG_KEY
    return None


def _load_room_entities(
    base_address: int,
    read_address: Callable[[int], int],
) -> List[DungeonSprite]:
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
        if read_address(sprite_address) == _stop_marker:
            break
        if remaining_max_bytes < 1:
            raise "Maximum bytes exceeded. Aborting to prevent infinite loop"

        byte0 = read_address(sprite_address)
        byte1 = read_address(sprite_address + 1)
        y = byte0 & _remove_subtype
        x = byte1 & _remove_overlord
        is_subtype = byte0 & _has_subtype == _has_subtype
        is_overlord = byte1 & _has_overlord == _has_overlord
        sprite_value = read_address(sprite_address + 2)
        type = SpriteId(sprite_value + (_overlord_offset if is_overlord else 0))
        item = _peek_item(sprite_address, read_address)

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


def _load_room(
    id: DungeonRoomId,
    read_address: Callable[[int], int],
) -> DungeonRoom:
    # Find the address of the Dungeon Room and read in graphics block and tags.
    address = resolve_address(
        read_address(_room_pointer_header_address + (id * 2)),
        read_address(_room_pointer_header_address + (id * 2) + 1),
        _room_bank_address,
    )
    # Read in the graphics block which controls the spritesheets
    # and tags which declare behaviors.
    lights_out = bool(read_address(address) & 0b1 == 0b1)
    palette_id = read_address(address + 1)
    tileset_id = DungeonRoomTilesetId(read_address(address + 2))
    blockset_id = SpriteBlocksetId.from_room_value(read_address(address + 3))
    effect = read_address(address + 4)
    tag1 = DungeonTag(read_address(address + 5))
    tag2 = DungeonTag(read_address(address + 6))

    sprite_table_base_snes_address = resolve_address(
        read_address(_sprite_pointer_table_address + (id * 2)),
        read_address(_sprite_pointer_table_address + (id * 2) + 1),
        _room_sprite_bank_address,
    )

    dungeon_sprites = _load_room_entities(
        sprite_table_base_snes_address,
        read_address,
    )

    return DungeonRoom(
        address,
        id,
        lights_out_effect=lights_out,
        palette_id=palette_id,
        tileset_id=tileset_id,
        blockset_id=blockset_id,
        effect=effect,
        tag1=tag1,
        tag2=tag2,
        dungeon_sprites=dungeon_sprites,
    )


def load_rooms(read_address: Callable[[int], int]) -> Dict[DungeonRoomId, DungeonRoom]:
    return {id: _load_room(id, read_address) for id in list(DungeonRoomId)}


def save_rooms(
    dungeon_room_dict: Dict[DungeonRoomId, DungeonRoom],
    write_address: Callable[[int, int], None],
) -> None:
    for dungeon_room in dungeon_room_dict.values():
        write_address(dungeon_room._address + 1, dungeon_room.palette_id)
        write_address(dungeon_room._address + 2, dungeon_room.tileset_id)
        write_address(
            dungeon_room._address + 3, dungeon_room.blockset_id.get_room_value()
        )
        write_address(dungeon_room._address + 4, dungeon_room.effect)
        write_address(dungeon_room._address + 5, dungeon_room.tag1)
        write_address(dungeon_room._address + 6, dungeon_room.tag2)

        # Rewrite new Dungeon Sprites.
        for dungeon_sprite in dungeon_room.dungeon_sprites:
            write_address(
                dungeon_sprite._address,
                dungeon_sprite.y | _has_subtype
                if dungeon_sprite.sprite_id == SpriteId.xE4_KEY
                else dungeon_sprite.y,
            )
            write_address(
                dungeon_sprite._address + 1,
                dungeon_sprite.x | _has_overlord
                if dungeon_sprite.sprite_id >= _overlord_offset
                else dungeon_sprite.x,
            )
            write_address(
                dungeon_sprite._address + 2,
                dungeon_sprite.sprite_id
                - (
                    _overlord_offset
                    if dungeon_sprite.sprite_id >= _overlord_offset
                    else 0
                ),
            )
