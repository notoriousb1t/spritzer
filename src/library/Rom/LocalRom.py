from typing import BinaryIO, Callable
from math import floor


def compute_snes_address(byte0: int, byte1: int, bank: int) -> int:
    """Translates a 24 bit relative pointer to SNES address"""
    return (bank << 16) | (byte1 << 8) | byte0


def compute_pc_address(address: int) -> int:
    """Converts an SNES address to a PC address"""
    return (address & 0x7FFF) + (floor(address / 2) & 0xFF8000)


def resolve_address(byte0: int, byte1: int, bank: int) -> int:
    """Resolves pointers to addresses"""
    return compute_pc_address(compute_snes_address(byte0, byte1, bank))



class LocalRom:
    read_address: Callable[[int], int]
    write_address: Callable[[int, int], None]
    room_header_bank = 0x04
    dungeon_room_pointer_header_address = 0x271E2
    dungeon_sprite_bank = 0x09
    dungeon_sprite_ptr_table_address = 0x4D62E
    overworld_sprite_bank = 0x09
    overworld_sprite_pointer_table_address = 0x04C901
    sprite_blockset_address = 0x5B97
    sprite_setting_address = 0x06B080

    @property
    def sprite_setting_0_address(self) -> int:
        return self.sprite_setting_address

    @property
    def sprite_health_address(self) -> int:
        return self.sprite_setting_address + 0xF3

    @property
    def sprite_damage_address(self) -> int:
        return self.sprite_setting_address + (0xF3 * 2)

    @property
    def sprite_setting_3_address(self) -> int:
        return self.sprite_setting_address + (0xF3 * 3)

    @property
    def sprite_setting_4_address(self) -> int:
        return self.sprite_setting_address + (0xF3 * 4)

    @property
    def sprite_setting_5_address(self) -> int:
        return self.sprite_setting_address + (0xF3 * 5)

    def __init__(self, read_address: Callable[[int], int]) -> None:
        self.read_address = read_address
        self.write_address = None

    def enable_write_mode(self, write_address: Callable[[int, int], None]) -> None:
        # Remove reading to prevent interleaving reads and writes.
        self.read_address = None
        self.write_address = write_address


def get_local_rom(read: Callable[[int], int]) -> LocalRom:
    rom = LocalRom(read)
    rom.room_header_bank = read(0x0B5E7) # Vanilla = 0x04 | Enemizer = 0x36
    return rom
