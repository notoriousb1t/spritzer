from enum import Enum
from typing import List

from ..Model import SpriteSettings, SpriteId, SpriteVulnerability


class Placement(Enum):
    AREA = 1
    ROOM = 2
    KILL_ROOM = 3


def _is_classification_compatible(
    source_meta: SpriteSettings,
    target_meta: SpriteSettings,
) -> bool:
    return (
        source_meta.role == target_meta.role
        # Only replace aquatic things with aquatic things.
        and source_meta.is_aquatic == target_meta.is_aquatic
        # Flying creatures may have incompatible placement with other types, so only
        # replace flying creatures with flying creatures
        and source_meta.is_flying == target_meta.is_flying
    )


def is_compatible(
    source: SpriteId,
    target: SpriteId,
    placement: Placement,
    has_key=False,
) -> bool:
    """True if the source can be replaced with the target"""
    source_meta = source.meta()
    target_meta = target.meta()

    if not _is_classification_compatible(source_meta, target_meta):
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
