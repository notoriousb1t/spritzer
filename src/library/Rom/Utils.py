from math import floor
from typing import BinaryIO

def compute_snes_address(byte0: int, byte1: int, bank: int) -> int:
    """Translates a 24 bit relative pointer to SNES address"""
    return (bank << 16) | (byte1 << 8) | byte0


def compute_pc_address(address: int) -> int:
    """Converts an SNES address to a PC address"""
    return (address & 0x7FFF) + (floor(address / 2) & 0xFF8000)


def resolve_address(byte0: int, byte1: int, bank: int) -> int:
    """Resolves pointers to addresses"""
    return compute_pc_address(compute_snes_address(byte0, byte1, bank))

def read_snes_rom(stream: BinaryIO, strip_header: bool = True) -> bytearray:
    """Reads rom into bytearray and optionally strips off any smc header"""
    buffer = bytearray(stream.read())
    if strip_header and len(buffer) % 0x400 == 0x200:
        return buffer[0x200:]
    return buffer