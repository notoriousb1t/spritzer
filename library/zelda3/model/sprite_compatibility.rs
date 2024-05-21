use super::can_place_in_dw;
use super::can_place_in_lw;
use super::Sprite;
use super::HORIZONTAL;
use super::VERTICAL;
use crate::zelda3::model::can_hold_key;
use crate::zelda3::model::can_place_in_ow;
use crate::zelda3::model::can_place_in_uw;
use crate::zelda3::model::can_sprite_fly;
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

#[derive(PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub(crate) enum Rule {
    KeyRequired,
    KillRequired,
    Overworld,
    ReduceDifficulty,
    Underworld,
    DarkWorld,
    LightWorld,
}

/// True if all sprites from the source list can can be replaced with something from the target
/// list.
pub(crate) fn is_list_compatible(
    sprites: &Vec<Sprite>,
    choices: &Vec<SpriteId>,
    rules: &[Rule],
) -> bool {
    sprites.iter().all(|sprite| {
        let mut sprite_rules = rules.to_vec();
        if sprite.has_key() {
            sprite_rules.push(Rule::KeyRequired);
        }
        filter_compatible(sprite.id, &choices, &sprite_rules).len() > 0
    })
}

pub(crate) fn filter_compatible(
    sprite_id: SpriteId,
    choices: &[SpriteId],
    rules: &[Rule],
) -> Vec<SpriteId> {
    choices
        .iter()
        .filter(|it| {
            is_fully_compatible(&sprite_id, it, &rules)
        })
        .map(|it| *it)
        .collect::<Vec<_>>()
}

/// True if the source can be replaced with the target.
fn is_fully_compatible(source: &SpriteId, target: &SpriteId, rules: &[Rule]) -> bool {
    if source == target {
        return true;
    }
    if !is_classification_fully_compatible(source, target) {
        return false;
    }
    if !is_movement_compatible(source, target) {
        return false;
    }

    if rules.contains(&Rule::DarkWorld) && !can_place_in_dw(source) || !can_place_in_dw(target)
    {
        return false;
    }
    if rules.contains(&Rule::LightWorld) && !can_place_in_lw(source) || !can_place_in_lw(target)
    {
        return false;
    }
    if rules.contains(&Rule::Overworld) {
        return can_place_in_ow(source) && can_place_in_ow(target);
    }

    if !(can_place_in_uw(source) && can_place_in_uw(target)) {
        return false;
    }

    if rules.contains(&Rule::KeyRequired) && can_hold_key(source) {
        return can_hold_key(target)
            && get_sprite_vulnerability(source) == get_sprite_vulnerability(target);
    }

    if rules.contains(&Rule::KillRequired)
        || get_sprite_vulnerability(source) == SpriteVulnerability::Invulnerable
    {
        return get_sprite_vulnerability(source) == get_sprite_vulnerability(target);
    }
    get_sprite_vulnerability(target) != SpriteVulnerability::Invulnerable
}

fn is_classification_fully_compatible(source: &SpriteId, target: &SpriteId) -> bool {
    let source_type = get_sprite_type(source);
    let target_type = get_sprite_type(target);

    if source_type == SpriteType::Object
        || source_type == SpriteType::Npc
        || source_type == SpriteType::Overlord
        || source_type == SpriteType::Boss
    {
        return source == target;
    }

    if source_type != target_type {
        // Allow certain types to morph as needed.
        let is_coercible = match source_type {
            SpriteType::Creature => match target_type {
                SpriteType::Enemy => true,
                SpriteType::Hazard => true,
                SpriteType::Absorbable => true,
                _ => false,
            },
            SpriteType::Hazard => match target_type {
                SpriteType::Enemy => true,
                SpriteType::Creature => true,
                _ => false,
            },
            SpriteType::Enemy => match target_type {
                SpriteType::Creature => true,
                _ => false,
            },
            _ => false,
        };

        if !is_coercible {
            return false;
        }
    }

    // Only replace aquatic things with aquatic things.
    return can_sprite_swim(source) == can_sprite_swim(target)
        // Flying creatures may have incompatible placement with other types, so only
        // replace flying creatures with flying creatures
        && can_sprite_fly(source) == can_sprite_fly(target);
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
