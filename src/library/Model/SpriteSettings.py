from .SpriteType import SpriteType
from .SpriteVulnerability import SpriteVulnerability
from .SpriteMovement import SpriteMovement


class SpriteSettings:
    """Holds debugging information and flags about the Entity."""

    vulnerability: SpriteVulnerability
    """Describes how to harm this Sprite."""
    can_shuffle_in_area: bool
    """True if this Sprite is allowed to be shuffled in Overworld Areas."""
    can_shuffle_in_room: bool
    """True if this Sprite is allowed to be shuffled in Underworld Rooms."""
    can_hold_key: bool
    """True if the Sprite can be assigned a key."""
    is_aquatic: bool
    """True if the Sprite swims. This helps keeps ground enemies from spawning in the water"""
    is_flying: bool
    """True if the Sprite flies. This signals if the enemy may is in a position a ground enemy
    would get stuck on."""
    role: SpriteType
    """The role of the Entity in the world. Determines randomization strategies."""
    movement: SpriteMovement
    """The possible movements for this sprite. This was created largely for hazards."""

    def __init__(
        self,
        role=SpriteType.UNSPECIFIED,
        is_aquatic=False,
        is_flying=False,
        vulnerability=None,
        can_shuffle=None,
        can_shuffle_in_area=None,
        can_shuffle_in_room=None,
        can_hold_key=None,
        movement=None,
    ) -> None:
        self.role = role
        self.is_aquatic = is_aquatic
        self.is_flying = is_flying
        self.movement = movement

        # By default, all creatures, enemies, and consumables can be shuffled.
        can_shuffle = (
            can_shuffle
            if can_shuffle != None
            else role
            in [
                SpriteType.CREATURE,
                SpriteType.ENEMY,
                SpriteType.HAZARD,
                SpriteType.CONSUMABLE,
            ]
        )
        self.can_shuffle_in_area = (
            can_shuffle if can_shuffle_in_area == None else can_shuffle_in_area
        )
        self.can_shuffle_in_room = (
            can_shuffle if can_shuffle_in_room == None else can_shuffle_in_room
        )

        if role == SpriteType.ENEMY or role == SpriteType.BOSS:
            # Set defaults that preserve underworld progression.
            self.can_hold_key = can_hold_key if can_hold_key != None else True
            self.vulnerability = (
                vulnerability if vulnerability != None else SpriteVulnerability.ANY
            )
        else:
            self.can_hold_key = False
            self.vulnerability = SpriteVulnerability.INVULNERABLE