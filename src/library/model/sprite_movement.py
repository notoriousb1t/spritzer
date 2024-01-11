from enum import IntFlag


class SpriteMovement(IntFlag):
    FIXED = 1 << 0
    EAST = 1 << 1
    WEST = 1 << 2
    NORTH = 1 << 3
    SOUTH = 1 << 4
    DIAGONAL = 1 << 5
    SNAKE = 1 << 6
    VERTICAL = EAST | WEST
    HORIZONTAL = NORTH | SOUTH

    def __str__(self) -> str:
        return self.name
