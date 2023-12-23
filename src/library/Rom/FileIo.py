from typing import BinaryIO


def read_snes_rom(stream: BinaryIO, strip_header: bool = True) -> bytearray:
    """Reads rom into bytearray and optionally strips off any smc header"""
    buffer = bytearray(stream.read())
    if strip_header and len(buffer) % 0x400 == 0x200:
        return buffer[0x200:]
    return buffer


class FileIo:
    """Handles read/write operations for standalone patching. This is unused when integrated into another Randomizer"""

    buffer: bytearray

    def __init__(self, input_file: str) -> None:
        self.hash = hash

        with open(input_file, "rb") as stream:
            self.buffer = read_snes_rom(stream)

    def read_byte(self, address: int) -> int:
        return self.buffer[address]

    def write_byte(self, address: int, value: int) -> None:
        self.buffer[address] = value

    def write_to_file(self, output_file: str) -> None:
        with open(output_file, "wb") as outfile:
            outfile.write(self.buffer)
