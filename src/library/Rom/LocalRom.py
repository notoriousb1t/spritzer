from .Utils import read_snes_rom


class LocalRom:
    """Handles read/write operations."""
    buffer: bytearray
    
    def __init__(self, input_file: str) -> None:
        self.hash = hash

        with open(input_file, 'rb') as stream:
            self.buffer = read_snes_rom(stream)

    def read_byte(self, address: int) -> int:
        return self.buffer[address]

    def write_byte(self, address: int, value: int) -> None:
        self.buffer[address] = value

    def write_to_file(self, output_file: str) -> None:
        with open(output_file, 'wb') as outfile:
            outfile.write(self.buffer)
