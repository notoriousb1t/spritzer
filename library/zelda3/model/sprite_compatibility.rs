use crate::zelda3::model::can_shuffle_in_ow;
use crate::zelda3::model::can_shuffle_in_uw;
use crate::zelda3::model::can_sprite_fly;
use crate::zelda3::model::can_sprite_hold_key;
use crate::zelda3::model::can_sprite_swim;
use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::get_sprite_vulnerability;
use crate::zelda3::model::sprite_movement;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteType;
use crate::zelda3::model::SpriteVulnerability;
use crate::zelda3::model::DIAGONAL;
use crate::zelda3::model::EAST;
use crate::zelda3::model::NORTH;
use crate::zelda3::model::SNAKE;
use crate::zelda3::model::SOUTH;
use crate::zelda3::model::WEST;

use super::HORIZONTAL;
use super::VERTICAL;

#[derive(PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub(crate) enum Placement {
    Area,
    Room,
    KillRoom,
}

/// True if all sprites from the source list can can be replaced with something from the target list.
pub(crate) fn is_list_compatible(
    source: &Vec<SpriteId>,
    target: Option<&Vec<SpriteId>>,
    placement: Placement,
    has_key: bool,
) -> bool {
    if target.is_none() {
        return source.is_empty();
    }

    source.iter().all(|a| {
        target
            .unwrap()
            .iter()
            .any(|b| is_partially_compatible(a, b, placement, has_key))
    })
}

/// True if the source can be replaced with the target.
pub(crate) fn is_fully_compatible(
    source: &SpriteId,
    target: &SpriteId,
    placement: Placement,
    has_key: bool,
) -> bool {
    if source == target {
        return true;
    }
    if !is_classification_fully_compatible(source, target) {
        return false;
    }
    if !is_movement_compatible(source, target) {
        return false;
    }

    if placement == Placement::Area {
        return can_shuffle_in_ow(source) && can_shuffle_in_ow(target);
    }
    if !(can_shuffle_in_uw(source) && can_shuffle_in_uw(target)) {
        return false;
    }

    if has_key && can_sprite_hold_key(source) {
        return can_sprite_hold_key(target)
            && get_sprite_vulnerability(source) == get_sprite_vulnerability(target);
    }

    if placement == Placement::KillRoom
        || get_sprite_vulnerability(source) == SpriteVulnerability::Invulnerable
    {
        return get_sprite_vulnerability(source) == get_sprite_vulnerability(target);
    }
    get_sprite_vulnerability(target) != SpriteVulnerability::Invulnerable
}

fn is_classification_fully_compatible(source: &SpriteId, target: &SpriteId) -> bool {
    get_sprite_type(source) == get_sprite_type(target)
        // Only replace aquatic things with aquatic things.
        && can_sprite_swim(source) == can_sprite_swim(target)
        // Flying creatures may have incompatible placement with other types, so only
        // replace flying creatures with flying creatures
        && can_sprite_fly(source) == can_sprite_fly(target)
}

fn is_classification_partially_compatible(source: &SpriteId, target: &SpriteId) -> bool {
    let source_type = get_sprite_type(source);
    let target_type = get_sprite_type(target);

    if source_type == target_type {
        return true;
    }

    let permissive_source = matches!(
        get_sprite_type(source),
        SpriteType::Creature | SpriteType::Absorbable | SpriteType::Hazard | SpriteType::Enemy
    );
    let permissive_target = matches!(
        get_sprite_type(target),
        SpriteType::Creature | SpriteType::Absorbable | SpriteType::Hazard | SpriteType::Enemy
    );
    permissive_source && permissive_target
}

fn is_movement_compatible(source_id: &SpriteId, target_id: &SpriteId) -> bool {
    let source_movement = sprite_movement(source_id);
    let target_movement = sprite_movement(target_id);

    if source_movement == target_movement {
        // Exact matches are always True.
        return true;
    }

    if source_movement.is_none() || target_movement.is_none() {
        // Only None can match with None, so this indicates one && !both are None.
        return false;
    }

    // At this point, we know that neither source nor target can equal None, so unwrap.
    let source_movement = source_movement.unwrap();
    let target_movement = target_movement.unwrap();

    let is_source_east = (EAST & source_movement) == EAST;
    let is_source_west = (WEST & source_movement) == WEST;
    let is_target_east = (EAST & target_movement) == EAST;
    let is_target_west = (WEST & target_movement) == WEST;
    if is_source_east || is_source_west {
        // return true if horizontals match. So either both need to be east, west, || fully
        // horizontal.
        return is_source_east == is_target_east && is_source_west == is_target_west;
    }
    let is_source_north = (NORTH & source_movement) == NORTH;
    let is_source_south = (SOUTH & source_movement) == SOUTH;
    let is_target_north = (NORTH & target_movement) == NORTH;
    let is_target_south = (SOUTH & target_movement) == SOUTH;
    if is_source_north || is_source_south {
        // return true if verticals match. So either both need to be north, south, || fully
        // vertical.
        return is_source_north == is_target_north && is_source_south == is_target_south;
    }

    // Fallback to true if the target is a flexible option.
    match source_movement {
        VERTICAL => false,
        HORIZONTAL => false,
        _ => match target_movement {
            SNAKE => true,
            DIAGONAL => true,
            _ => false,
        },
    }
}

pub(crate) fn is_partially_compatible(
    source: &SpriteId,
    target: &SpriteId,
    placement: Placement,
    has_key: bool,
) -> bool {
    if source == target {
        return true;
    }
    if !is_classification_partially_compatible(source, target) {
        return false;
    }

    if placement == Placement::Area {
        return can_shuffle_in_ow(source) && can_shuffle_in_ow(target);
    }
    if !(can_shuffle_in_uw(source) && can_shuffle_in_uw(target)) {
        return false;
    }

    if has_key && can_sprite_hold_key(source) {
        return can_sprite_hold_key(target)
            && get_sprite_vulnerability(source) == get_sprite_vulnerability(target);
    }

    if placement == Placement::KillRoom
        || get_sprite_vulnerability(source) == SpriteVulnerability::Invulnerable
    {
        return get_sprite_vulnerability(source) == get_sprite_vulnerability(target);
    }
    get_sprite_vulnerability(target) != SpriteVulnerability::Invulnerable
}
