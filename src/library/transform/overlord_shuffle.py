from random import Random
from library.model.sprite_id import SpriteId
from library.model.model import Model
from library.rom.local_rom import LocalRom


BNE = 0xD0
BEQ = 0xF0
NOP = 0xEA
LDA_b = 0xA9
AND_b = 0x25
NEGATIVE_MASK = 0b1100_0000


def _replace_cannon_overlord_with_bees(context: Model, rom: LocalRom) -> None:
    # Increase the time between bees. This reduces how many bees are spawned all at once.
    rom.write_snes_address(snes_address=0x9BF83, value=0x64)
    # Swap all cannon ball overlords with bees.
    rom.write_snes_address(snes_address=0x9BFB0, value=SpriteId.x79_BEE)


def _replace_cannon_overlord_with_big_cannons(context: Model, rom: LocalRom) -> None:
    # Increase the time between balls. This allows more time to go
    # between the larger balls. Without this modification, it is impossible to
    # avoid damage without cape, cane, etc. This value is normally 0x38 ticks.
    rom.write_snes_address(snes_address=0x9BF83, value=0x80)
    # Always use the larger ball
    rom.write_snes_address(snes_address=0x9BF8A, value=BEQ)
    # Increase cannonball speed. this also affects the 4-way canon room.
    speed = 36
    rom.write_snes_bytes(
        snes_address=0x9C01A, values=[speed, speed | NEGATIVE_MASK, 0, 0]
    )
    rom.write_snes_bytes(
        snes_address=0x9C01A, values=[0, 0, speed, speed | NEGATIVE_MASK]
    )


def _noop(context: Model, rom: LocalRom) -> None:
    return


def reroll_overlords(context: Model, rom: LocalRom) -> None:
    random = Random(x=context.seed)

    randomize_cannon_overlord = random.choice(
        seq=[
            _replace_cannon_overlord_with_bees,
            _replace_cannon_overlord_with_big_cannons,
            _noop,
        ]
    )
    randomize_cannon_overlord(context=context, rom=rom)


def reroll_eastern_palace_overlords(context: Model, rom: LocalRom) -> None:
    random = Random()
    random.seed(a=context.seed)
