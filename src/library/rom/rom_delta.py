from attr import dataclass


@dataclass
class RomDelta:
    pc_address: int
    snes_address: int
    original_value: int
    current_value: int

    def __str__(self) -> str:
        return f"Modified {hex(self.snes_address)} ({self.pc_address}): {hex(self.original_value)} -> {hex(self.current_value)}"