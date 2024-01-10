from enum import IntEnum


class SpriteType(IntEnum):
    """Describes the general archetype/role of the Sprite. This is only used for randomization and is a construct of this library."""

    UNSPECIFIED = 1  # Unclear or not applicable to randomization
    BOSS = 2  # Boss instances. Some bosses also have enemy spawns as well.
    CREATURE = 3  # Creatures like Cuccos and rabbits.
    ENEMY = 4  # Most things that harm you.
    NPC = 5  # Invincible characters such as townfolk.
    OBJECT = 6  # Basic category for interactive objects such as projectiles.
    OVERLORD = 7  # Screen events like Bee traps.
    CONSUMABLE = 8  # Consumed by Link or temporary, such as rupees.
    HAZARD = 9  # Fire bars, spikes, etc.

    def __str__(self) -> str:
        return self.name
