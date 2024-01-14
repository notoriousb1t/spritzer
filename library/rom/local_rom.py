from math import floor
from typing import List
from library.rom.rom_delta import RomDelta
from library.rom.rom_mode import RomMode


def snes_address_to_bytes(address: int) -> List[int]:
    return [(address >> 16) & 0xFF, (address >> 8) & 0xFF, address & 0xFF]


def pc_address_to_snes_address(pc_address: int) -> int:
    # The integer in big endian bytes.
    int_bytes: list[int] = [
        (pc_address >> 16) & 0xFF,
        (pc_address >> 8) & 0xFF,
        pc_address & 0xFF,
    ]
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
    snes_address: int = compute_snes_address(byte_values=byte_values)
    return compute_pc_address(address=snes_address)


def resolve_snes_pointer(pc_address: int) -> List[int]:
    snes_address: int = pc_address_to_snes_address(pc_address=pc_address)
    return snes_address_to_bytes(address=snes_address)


class LocalRom:
    """Manages reading and writing to the Rom data.

    NOTE: The following conventions are used for pointers/byte address:
          suffix of snes (preferred) indicates an SNES address
          suffix of pc indicates a PC address
          suffix of bank indicates an SNES bank
    """

    _original_buffer: bytearray
    _current_buffer: bytearray
    _mode = RomMode.READ

    room_header_bank = 0x04
    overworld_sprite_bank = 0x09
    underworld_sprite_bank = 0x09

    sprite_blockset_snes = 0x00DB97
    room_header_pointers_snes = 0x4F1E2

    area_special_graphics_snes = 0x2E576
    area_special_palette_snes = 0x2E596
    area_sprite_pointers_lightworld_v1_snes = 0x9C901
    area_sprite_pointers_lightworld_v2_snes = 0x9CA21
    area_sprite_pointers_darkworld_v2_snes = 0x9CAA1
    area_sprite_pointers_specialworld_v2_snes = 0x9CB21
    area_sprite_empty = 0x9CB41

    room_sprite_pointers_snes = 0x9D62E
    damage_table_snes = 0x06F42D
    area_graphics_snes = 0xFA41
    weapon_damage_snes = 0xDB8F1

    @property
    def sprite_settings0_snes(self) -> int:
        return 0xDB080

    @property
    def sprite_settings1_snes(self) -> int:
        return self.sprite_settings0_snes + 0xF3  # 0xD_B173

    @property
    def sprite_settings2_snes(self) -> int:
        return self.sprite_settings0_snes + (0xF3 * 2)  # 0xD_B266

    @property
    def sprite_settings3_snes(self) -> int:
        return self.sprite_settings0_snes + (0xF3 * 3)  # 0xD_B359

    @property
    def sprite_settings4_snes(self) -> int:
        return self.sprite_settings0_snes + (0xF3 * 4)  # 0xD_B44C

    @property
    def sprite_settings5_snes(self) -> int:
        return self.sprite_settings0_snes + (0xF3 * 5)  # 0xD_B53F

    @property
    def sprite_settings6_snes(self) -> int:
        return self.sprite_settings0_snes + (0xF3 * 6)  # 0xD_B632

    @property
    def sprite_settings7_snes(self) -> int:
        return self.sprite_settings0_snes + (0xF3 * 7)  # 0xD_B725

    def __init__(self, buffer: bytearray) -> None:
        self._original_buffer = buffer.copy()
        self._current_buffer = buffer

    def set_mode(self, mode: RomMode) -> None:
        self._mode: RomMode = self._mode.next(value=mode)

    def read_address(self, address: int) -> int:
        if self._mode != RomMode.READ:
            raise "Wrong phase, cannot read"  # type: ignore
        return self._current_buffer[address]

    def read_snes_address(self, snes_address: int) -> int:
        return self.read_address(address=compute_pc_address(address=snes_address))

    def write_address(self, address: int, value: int) -> None:
        if self._mode != RomMode.WRITE:
            raise "Wrong phase, cannot write"  # type: ignore
        self._current_buffer[address] = value

    def write_bytes(self, address: int, values: List[int]) -> None:
        if self._mode != RomMode.WRITE:
            raise "Wrong phase, cannot write"  # type: ignore
        for i, val in enumerate(iterable=values):
            self._current_buffer[address + i] = val

    def write_snes_address(self, snes_address: int, value: int) -> None:
        return self.write_address(
            address=compute_pc_address(address=snes_address),
            value=value,
        )

    def write_snes_bytes(self, snes_address: int, values: List[int]) -> None:
        return self.write_bytes(
            address=compute_pc_address(address=snes_address),
            values=values,
        )

    def write_crc(self) -> None:
        if self._mode != RomMode.CRC:
            raise "Wrong phase, cannot add CRC"  # type: ignore
        crc: int = (
            sum(self._current_buffer[:0x7FDC] + self._current_buffer[0x7FE0:]) + 0x01FE
        ) & 0xFFFF

        inv = crc ^ 0xFFFF
        crc_bytes: list[int] = [
            inv & 0xFF,
            (inv >> 8) & 0xFF,
            crc & 0xFF,
            (crc >> 8) & 0xFF,
        ]
        for i, value in enumerate(iterable=crc_bytes):
            self._current_buffer[0x7FDC + i] = value

    def get_deltas(self) -> List[RomDelta]:
        deltas: List[RomDelta] = []
        for i, _ in enumerate(iterable=self._original_buffer):
            original_value: int = self._original_buffer[i]
            current_value: int = self._current_buffer[i]
            if current_value != original_value:
                deltas.append(
                    RomDelta(
                        pc_address=i,
                        snes_address=pc_address_to_snes_address(i),
                        original_value=original_value,
                        current_value=current_value,
                    )
                )
        return deltas


def get_local_rom(buffer: bytearray) -> LocalRom:
    rom = LocalRom(buffer=buffer)
    rom.room_header_bank = rom.read_address(
        address=0x0B5E7
    )  # Vanilla = 0x04 | Enemizer = 0x36
    return rom
