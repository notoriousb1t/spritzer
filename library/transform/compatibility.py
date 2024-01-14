from enum import Enum
from library.model.sprite_id import SpriteId
from library.model.sprite_movement import SpriteMovement
from library.model.sprite_configuration import SpriteConfiguration
from library.model.sprite_vulnerability import SpriteVulnerability


class Placement(Enum):
    AREA = 1
    ROOM = 2
    KILL_ROOM = 3


def _is_classification_compatible(
    source_meta: SpriteConfiguration,
    target_meta: SpriteConfiguration,
) -> bool:
    return (
        source_meta.role == target_meta.role
        # Only replace aquatic things with aquatic things.
        and source_meta.is_aquatic == target_meta.is_aquatic
        # Flying creatures may have incompatible placement with other types, so only
        # replace flying creatures with flying creatures
        and source_meta.is_flying == target_meta.is_flying
    )


def is_movement_compatible(
    source: SpriteConfiguration,
    target: SpriteConfiguration,
) -> bool:
    if source.movement == target.movement:
        # Exact matches are always True.
        return True

    if source.movement == None or target.movement == None:
        # Only None can match with None, so this indicates one and not both are None.
        return False

    if any(
        it in source.movement and it in target.movement
        for it in [SpriteMovement.FIXED, SpriteMovement.DIAGONAL, SpriteMovement.SNAKE]
    ):
        # Check for direct matches of fixed, diagonal, and snake since they don't have special
        # combination aspects (like EAST and WEST have)
        return True

    is_source_east = SpriteMovement.EAST in source.movement
    is_source_west = SpriteMovement.WEST in source.movement
    is_target_east = SpriteMovement.EAST in target.movement
    is_target_west = SpriteMovement.WEST in target.movement
    if is_source_east or is_source_west:
        # Return true if horizontals match. So either both need to be east, west, or fully horizontal.
        return is_source_east == is_target_east and is_source_west == is_target_west

    is_source_north = SpriteMovement.NORTH in source.movement
    is_source_south = SpriteMovement.SOUTH in source.movement
    is_target_north = SpriteMovement.NORTH in target.movement
    is_target_south = SpriteMovement.SOUTH in target.movement
    if is_source_north or is_source_south:
        # Return true if verticals match. So either both need to be north, south, or fully vertical.
        return is_source_north == is_target_north and is_source_south == is_target_south

    return False


def is_compatible(
    source: SpriteId,
    target: SpriteId,
    placement: Placement,
    has_key: bool = False,
) -> bool:
    """True if the source can be replaced with the target"""
    source_meta: SpriteConfiguration = source.configuration()
    target_meta: SpriteConfiguration = target.configuration()

    if not _is_classification_compatible(
        source_meta=source_meta, target_meta=target_meta
    ):
        return False

    if not is_movement_compatible(source=source_meta, target=target_meta):
        return False

    if placement == Placement.AREA:
        return source_meta.can_shuffle_in_area and target_meta.can_shuffle_in_area

    if not (source_meta.can_shuffle_in_room and target_meta.can_shuffle_in_room):
        return False

    if has_key and source_meta.can_hold_key:
        return (
            target_meta.can_hold_key
            and source_meta.vulnerability == target_meta.vulnerability
        )

    if (
        placement == Placement.KILL_ROOM
        or source_meta.vulnerability == SpriteVulnerability.INVULNERABLE
    ):
        return source_meta.vulnerability == target_meta.vulnerability

    return target_meta.vulnerability != SpriteVulnerability.INVULNERABLE
