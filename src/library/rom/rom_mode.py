from enum import IntEnum, auto
from typing import Self


class RomMode(IntEnum):
    """This enforces architecture: read -> transform -> write"""

    READ = auto()
    LOCKED = auto()
    WRITE = auto()
    CRC = auto()

    def next(self, value: Self) -> Self:
        if RomMode(value=self.value + 1) != RomMode(value=value):
            raise "Not a valid phase change"  # type: ignore
        return value
