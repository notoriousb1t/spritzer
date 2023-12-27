from enum import IntEnum, auto
from typing import List
from math import floor


def snes_address_to_bytes(address: int) -> List[int]:
    return [(address >> 16) & 0xFF, (address >> 8) & 0xFF, address & 0xFF]


def pc_address_to_snes_address(pc_address: int) -> int:
    # The integer in big endian bytes.
    int_bytes = [(pc_address >> 16) & 0xFF, (pc_address >> 8) & 0xFF, pc_address & 0xFF]
    # Convert to SNES bytes.
    int_bytes[0] = max(0, min(int_bytes[0] * 2, 255))
    if int_bytes[1] >= 0x80:
        int_bytes[0] += 1
    else:
        int_bytes[1] += 0x80
    # Recombine into an integer.
    return (int_bytes[0] << 16) | (int_bytes[1] << 8) | int_bytes[2]


def compute_snes_address(byte_values: List[int]) -> int:
    return (byte_values[0] << 16) | (byte_values[1] << 8) | byte_values[2]


def compute_pc_address(address: int) -> int:
    return (address & 0x7FFF) + (floor(address / 2) & 0xFF8000)


def resolve_address(byte_values: List[int]) -> int:
    snes_address = compute_snes_address(byte_values)
    return compute_pc_address(snes_address)


def resolve_snes_pointer(pc_address: int) -> List[int]:
    snes_address = pc_address_to_snes_address(pc_address)
    return snes_address_to_bytes(snes_address)


class RomMode(IntEnum):
    """This enforces architecture: read -> transform -> write"""

    READ = auto()
    LOCKED = auto()
    WRITE = auto()
    CRC = auto()

    def next(self, value) -> None:
        if RomMode(self.value + 1) != RomMode(value):
            raise "Not a valid phase change"
        return value


class LocalRom:
    _buffer: bytearray
    _mode = RomMode.READ

    room_header_bank = 0x04
    dungeon_room_pointer_header_address = 0x271E2
    dungeon_sprite_bank = 0x09
    overworld_sprite_bank = 0x09
    sprite_blockset_address = 0x5B97
    sprite_setting_address = 0x06B080

    @property
    def dungeon_sprite_ptr_table_address(self) -> int:
        return compute_snes_address([0x4, 0xD6, 0x2E])  # 0x04D62E

    @property
    def overworld_sprite_ptr_table_address(self) -> int:
        return compute_snes_address([0x4, 0xC9, 0x01])  # 0x04C901

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

    def __init__(self, buffer: bytearray) -> None:
        self._buffer = buffer

    def set_mode(self, mode: RomMode) -> None:
        self._mode = self._mode.next(mode)

    def read_address(self, address: int) -> int:
        if self._mode != RomMode.READ:
            raise "Wrong phase, cannot read"
        return self._buffer[address]

    def write_address(self, address: int, value: int) -> None:
        if self._mode != RomMode.WRITE:
            raise "Wrong phase, cannot write"
        self._buffer[address] = value

    def write_crc(self):
        if self._mode != RomMode.CRC:
            raise "Wrong phase, cannot add CRC"
        crc = (sum(self._buffer[:0x7FDC] + self._buffer[0x7FE0:]) + 0x01FE) & 0xFFFF

        inv = crc ^ 0xFFFF
        crc_bytes = [inv & 0xFF, (inv >> 8) & 0xFF, crc & 0xFF, (crc >> 8) & 0xFF]
        for i, value in enumerate(crc_bytes):
            self._buffer[0x7FDC + i] = value


def get_local_rom(buffer: bytearray) -> LocalRom:
    rom = LocalRom(buffer)
    rom.room_header_bank = rom.read_address(0x0B5E7)  # Vanilla = 0x04 | Enemizer = 0x36
    print(hex(rom.overworld_sprite_ptr_table_address))
    return rom
