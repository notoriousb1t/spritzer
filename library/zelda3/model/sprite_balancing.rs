use std::collections::BTreeMap;

use crate::zelda3::model::get_sprite_challenge;
use crate::zelda3::model::get_sprite_type;
use crate::zelda3::model::SpriteChallenge;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::SpriteType;
use crate::zelda3::options::Balancing;

/// Provides weightings to enforce some sort of difficulty for sprite placement when rerolling.
pub(crate) fn get_weights(
    balancing: &Balancing,
    is_adjusted: bool,
    sprite_ids: &[&SpriteId],
) -> BTreeMap<SpriteId, usize> {
    let weight_fn: fn(&SpriteId) -> usize = match is_adjusted {
        true => match balancing {
            Balancing::Random => get_weights_random_adjusted,
            Balancing::Casual => get_weights_casual_adjusted,
            Balancing::Balanced => get_weights_balanced_adjusted,
            Balancing::Hero => get_weights_hero_adjusted,
        },
        false => match balancing {
            Balancing::Random => get_weights_random,
            Balancing::Casual => get_weights_casual,
            Balancing::Balanced => get_weights_balanced,
            Balancing::Hero => get_weights_hero,
        },
    };
    return BTreeMap::from_iter(
        sprite_ids
            .iter()
            .map(|&sprite_id| (*sprite_id, weight_fn(sprite_id))),
    );
}

// Provides the weighting for a single sprite based on the balancing parameter.
pub(crate) fn get_weight(balancing: Balancing, sprite_id: SpriteId) -> usize {
    let weight_fn = match balancing {
        Balancing::Random => get_weights_random,
        Balancing::Casual => get_weights_casual,
        Balancing::Balanced => get_weights_balanced,
        Balancing::Hero => get_weights_hero,
    };
    weight_fn(&sprite_id)
}

/// Equal chance for randomization. YOLO!
fn get_weights_random(sprite_id: &SpriteId) -> usize {
    match get_sprite_challenge(sprite_id) {
        SpriteChallenge::X0None => 0,
        _ => 1,
    }
}

/// YOLO except for adjusted areas like rescue and escape.
fn get_weights_random_adjusted(sprite_id: &SpriteId) -> usize {
    match get_sprite_challenge(sprite_id) {
        SpriteChallenge::X0None => 0,
        SpriteChallenge::X6Harder => 0,
        SpriteChallenge::X7Hardest => 0,
        _ => 1,
    }
}


/// Reduce the chance of a bunch of super hard enemies.
/// The name is a bit of a misnomer, but it is balanced according to
/// general player perception.

fn get_weights_balanced(sprite_id: &SpriteId) -> usize {
    match get_sprite_type(sprite_id) {
        SpriteType::Enemy => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 50,
            SpriteChallenge::X2Easy => 50,
            SpriteChallenge::X3Easier => 30,
            SpriteChallenge::X4Medium => 30,
            SpriteChallenge::X5Hard => 10,
            SpriteChallenge::X6Harder => 2,
            SpriteChallenge::X7Hardest => 1,
        },
        SpriteType::Hazard => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 5,
            SpriteChallenge::X2Easy => 5,
            SpriteChallenge::X3Easier => 5,
            SpriteChallenge::X4Medium => 5,
            SpriteChallenge::X5Hard => 4,
            SpriteChallenge::X6Harder => 2,
            SpriteChallenge::X7Hardest => 1,
        },
        _ => 0,
    }
}

/// Reduce the chance of a bunch of super hard enemies.
/// The name is a bit of a misnomer, but it is balanced according to
/// general player perception.

fn get_weights_balanced_adjusted(sprite_id: &SpriteId) -> usize {
    match get_sprite_type(sprite_id) {
        SpriteType::Enemy => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 70,
            SpriteChallenge::X2Easy => 70,
            SpriteChallenge::X3Easier => 30,
            SpriteChallenge::X4Medium => 30,
            SpriteChallenge::X5Hard => 0,
            SpriteChallenge::X6Harder => 0,
            SpriteChallenge::X7Hardest => 0,
        },
        SpriteType::Hazard => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 5,
            SpriteChallenge::X2Easy => 5,
            SpriteChallenge::X3Easier => 5,
            SpriteChallenge::X4Medium => 5,
            SpriteChallenge::X5Hard => 0,
            SpriteChallenge::X6Harder => 0,
            SpriteChallenge::X7Hardest => 0,
        },
        _ => 0,
    }
}

/// Reduce the chance of anything but easier enemies.
fn get_weights_casual(sprite_id: &SpriteId) -> usize {
    match get_sprite_type(sprite_id) {
        SpriteType::Enemy => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 1000,
            SpriteChallenge::X2Easy => 500,
            SpriteChallenge::X3Easier => 500,
            SpriteChallenge::X4Medium => 10,
            SpriteChallenge::X5Hard => 0,
            SpriteChallenge::X6Harder => 0,
            SpriteChallenge::X7Hardest => 0,
        },
        SpriteType::Hazard => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 10,
            SpriteChallenge::X2Easy => 10,
            SpriteChallenge::X3Easier => 10,
            SpriteChallenge::X4Medium => 1,
            SpriteChallenge::X5Hard => 1,
            SpriteChallenge::X6Harder => 1,
            SpriteChallenge::X7Hardest => 0,
        },
        _ => 0,
    }
}

fn get_weights_casual_adjusted(sprite_id: &SpriteId) -> usize {
    match get_sprite_type(sprite_id) {
        SpriteType::Enemy => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 1000,
            SpriteChallenge::X2Easy => 500,
            SpriteChallenge::X3Easier => 500,
            SpriteChallenge::X4Medium => 0,
            SpriteChallenge::X5Hard => 0,
            SpriteChallenge::X6Harder => 0,
            SpriteChallenge::X7Hardest => 0,
        },
        SpriteType::Hazard => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 10,
            SpriteChallenge::X2Easy => 10,
            SpriteChallenge::X3Easier => 10,
            SpriteChallenge::X4Medium => 0,
            SpriteChallenge::X5Hard => 0,
            SpriteChallenge::X6Harder => 0,
            SpriteChallenge::X7Hardest => 0,
        },
        _ => 0,
    }
}

/// Reduce the chance of anything easy.
fn get_weights_hero(sprite_id: &SpriteId) -> usize {
    match get_sprite_type(sprite_id) {
        SpriteType::Enemy => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 0,
            SpriteChallenge::X2Easy => 0,
            SpriteChallenge::X3Easier => 0,
            SpriteChallenge::X4Medium => 100,
            SpriteChallenge::X5Hard => 500,
            SpriteChallenge::X6Harder => 500,
            SpriteChallenge::X7Hardest => 1000,
        },
        SpriteType::Hazard => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 1,
            SpriteChallenge::X2Easy => 2,
            SpriteChallenge::X3Easier => 3,
            SpriteChallenge::X4Medium => 4,
            SpriteChallenge::X5Hard => 4,
            SpriteChallenge::X6Harder => 4,
            SpriteChallenge::X7Hardest => 5,
        },
        _ => 0,
    }
}


fn get_weights_hero_adjusted(sprite_id: &SpriteId) -> usize {
    match get_sprite_type(sprite_id) {
        SpriteType::Enemy => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 0,
            SpriteChallenge::X2Easy => 0,
            SpriteChallenge::X3Easier => 50,
            SpriteChallenge::X4Medium => 100,
            SpriteChallenge::X5Hard => 500,
            SpriteChallenge::X6Harder => 0,
            SpriteChallenge::X7Hardest => 0,
        },
        SpriteType::Hazard => match get_sprite_challenge(sprite_id) {
            SpriteChallenge::X0None => 0,
            SpriteChallenge::X1Easiest => 1,
            SpriteChallenge::X2Easy => 2,
            SpriteChallenge::X3Easier => 3,
            SpriteChallenge::X4Medium => 4,
            SpriteChallenge::X5Hard => 4,
            SpriteChallenge::X6Harder => 0,
            SpriteChallenge::X7Hardest => 0,
        },
        _ => 0,
    }
}
