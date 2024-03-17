use std::cmp::{max, min};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashSet};

use log;
use rand::distributions::{Distribution, WeightedError, WeightedIndex};
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use strum::IntoEnumIterator;

use crate::zelda3::model::{
    can_shuffle_in_uw, get_sprite_requirements, get_sprite_type, get_spritesheet_arrangements,
    get_weight, is_list_compatible, is_restricted_sprite, is_special_damage_sprite,
    is_spritesheet_permanent_uw, Placement, SpriteId, SpriteType, Spriteset, SpritesetId,
    SpritesheetId, UWRoomId, Z3Model,
};
use crate::zelda3::Balancing;

/// Attempt to re-arrange distribution of spritesets.
pub(crate) fn apply_uw_spriteset_shuffle(model: &mut Z3Model) {
    clear_optional_uw_spritesheets(model);

    let mut rng = model.create_rng();
    fill_spritesets(model, &mut rng);

    // Recalculate sprite pool. If this doesn't happen, the randomization is super disappointing
    // because most spritesets have nothing in them.
    model.prepare_sprite_pool();

    choose_new_spriteset(model, &mut rng);
}

/// Clear all non-essential spritesheets.
fn clear_optional_uw_spritesheets(model: &mut Z3Model) {
    // Clear unnecessary spritesheets from spritesets
    let sprites_map = compute_spriteset_requirements(model);
    for (spriteset_id, sprite_ids) in sprites_map.iter() {
        let spriteset = model
            .spritesets
            .get_mut(spriteset_id)
            .expect("Spriteset should exist");
        let sprite_ids = &sprite_ids.iter().map(|it| *it).collect::<Vec<_>>();
        spriteset.sheets = get_sprite_requirements(spriteset.sheets, &sprite_ids, true);
    }

    // Empty the available spritesets and fill each with emptiness. Such empty...
    while let Some(spriteset_id) = model.uw_spritesets_unused.pop() {
        model.spritesets.insert(
            spriteset_id,
            Spriteset {
                id: spriteset_id,
                sheets: [SpritesheetId::None; 4],
            },
        );
    }
}

fn compute_spriteset_requirements(model: &Z3Model) -> BTreeMap<SpritesetId, HashSet<SpriteId>> {
    // Get a map between spritesets and the actual sprites loaded using that spriteset.
    let mut map: BTreeMap<SpritesetId, HashSet<SpriteId>> = BTreeMap::new();
    for room_id in UWRoomId::iter() {
        // Insert/Get an entry into the map based on the spriteset used.
        let header = model.uw_headers.get(&room_id).expect("Header should exist");

        // Determine which placement strategy is required.
        let placement = match header.tag1.is_kill_room() || header.tag2.is_kill_room() {
            true => Placement::KillRoom,
            false => Placement::Room,
        };

        // Get all required sprites from the room.
        let required_sprites = model
            .uw_sprites
            .get(&room_id)
            .expect("Sprites should exist")
            .sprites
            .iter()
            .map(|sprite| sprite.id)
            .filter(|sprite_id| {
                is_restricted_sprite(sprite_id)
                    || (placement == Placement::KillRoom && is_special_damage_sprite(sprite_id))
            })
            .collect::<Vec<_>>();
        let required_sprite_ids = match map.entry(header.spriteset_id) {
            Entry::Vacant(it) => it.insert(HashSet::new()),
            Entry::Occupied(it) => it.into_mut(),
        };
        required_sprite_ids.extend(required_sprites);
    }
    map
}

fn fill_spritesets(model: &mut Z3Model, rng: &mut StdRng) {
    // UW can only access a subset of spritesets.
    let spritesets = SpritesetId::iter()
        .filter(|it| it.is_underworld())
        .collect::<Vec<_>>();

    let mut spritesheet_pool = create_spritesheet_pool(model.uw_balancing);

    for spriteset_id in spritesets {
        let spriteset = model
            .spritesets
            .get_mut(&spriteset_id)
            .expect("Spriteset should exist");

        let is_spriteset_full = spriteset
            .sheets
            .iter()
            .all(|spritesheet| *spritesheet != SpritesheetId::None);
        if is_spriteset_full {
            // Skip sprite sheets with no slots.
            continue;
        }

        if fill_spriteset_from_pool(spriteset, rng, &mut spritesheet_pool) {
            continue;
        }

        // If there are no suitable sheets left over, refill the pool and try a second time.
        spritesheet_pool = create_spritesheet_pool(model.uw_balancing);
        if fill_spriteset_from_pool(spriteset, rng, &mut spritesheet_pool) {
            continue;
        }

        log::error!("Unable to fill spriteset {}", spriteset_id);
    }
}

fn create_spritesheet_pool(balancing: Balancing) -> Vec<(usize, [SpritesheetId; 4])> {
    SpriteId::iter()
        .filter(|it| {
            !is_restricted_sprite(it)
                && matches!(
                    get_sprite_type(it),
                    SpriteType::Enemy
                        | SpriteType::Hazard
                        | SpriteType::Absorbable
                        | SpriteType::Creature
                )
                && can_shuffle_in_uw(&it)
        })
        .flat_map(|sprite_id| {
            let weight = get_weight(balancing, sprite_id);
            get_spritesheet_arrangements(&sprite_id)
                .iter()
                .map(|spritesheets| (weight, *spritesheets))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

/// Fills a spriteset with spritesheets from the provided pools of sprites.
fn fill_spriteset_from_pool(
    spriteset: &mut Spriteset,
    rng: &mut StdRng,
    pool: &mut Vec<(usize, [SpritesheetId; 4])>,
) -> bool {
    for _ in 0..128 {
        if let Ok(spritesheets) = choose_from_pool(pool, rng) {
            let is_compatible = spritesheets.iter().enumerate().all(|(i, spritesheet)| {
                spriteset.sheets[i] == *spritesheet
                    || SpritesheetId::None == *spritesheet
                    || SpritesheetId::None == spriteset.sheets[i]
            });

            if !is_compatible {
                continue;
            }

            for i in 0..4 {
                if spritesheets[i] != SpritesheetId::None {
                    // Set all spritesheets in their respective slots.
                    spriteset.sheets[i] = spritesheets[i];
                }
            }

            let is_spriteset_full = spriteset
                .sheets
                .iter()
                .all(|spritesheet| *spritesheet != SpritesheetId::None);
            if is_spriteset_full {
                // Check once the spritesheets are set if all spritesheets are filled.
                return true;
            }
        }
    }
    false
}

fn choose_from_pool(
    pool: &mut Vec<(usize, [SpritesheetId; 4])>,
    rng: &mut StdRng,
) -> Result<[SpritesheetId; 4], WeightedError> {
    let weights = pool
        .iter()
        .map(|(weight, _)| *weight)
        .collect::<Vec<_>>();
    let weighted_index = WeightedIndex::new(&weights)?;
    let selected_index = weighted_index.sample(rng);
    let (_, spriteset) = pool.remove(selected_index);
    Ok(spriteset)
}

fn choose_new_spriteset(model: &mut Z3Model, rng: &mut StdRng) {
    let spritesets = SpritesetId::iter()
        .filter(|it| it.is_underworld())
        .collect::<Vec<_>>();

    // Create weights to prevent the same spriteset from being over-used. This encourages
    // every spriteset to be somewhat equal in distribution (although not guaranteed).
    // Each time a spriteset is used, it diminishes weight by half with a minimum of 1
    // so that special damage kill rooms always have a choice.
    const WEIGHT_FULL: usize = 16;
    const WEIGHT_EMPTY: usize = 1;
    let mut weights = BTreeMap::from_iter(
        spritesets
            .iter()
            .map(|spriteset_id| (*spriteset_id, WEIGHT_FULL)),
    );

    for uw_room_id in UWRoomId::iter() {
        let header = model
            .uw_headers
            .get_mut(&uw_room_id)
            .expect(&format!("UWRoomHeader to exist {}", uw_room_id));

        if let Some(permanent_spritesheet_id) = model
            .spritesets
            .get(&header.spriteset_id)
            .expect("Spriteset should exist")
            .sheets
            .iter()
            .find(|it| is_spritesheet_permanent_uw(it))
        {
            // Check if the spriteset is locked in place and skip it.
            // There are certain sprites that either aren't present in the room sprite data
            // or use different spritesheet offsets.
            log::debug!(
                "UW ${:02X} -- found permanent ${:02X} = ${:02X}",
                uw_room_id as u8,
                header.spriteset_id as u8,
                *permanent_spritesheet_id as u8,
            );
            continue;
        }

        // Get the sprites from the room this header is for.
        let uw_sprites = &model
            .uw_sprites
            .get(&uw_room_id)
            .expect("UW Header to have spritelist")
            .sprites;

        // Get a list of the sprite ids dereferenced.
        let existing_sprite_ids = uw_sprites
            .iter()
            .map(|sprite| sprite.id)
            .collect::<Vec<_>>();

        let has_special_requirements = existing_sprite_ids
            .iter()
            .any(|sprite_id| is_restricted_sprite(sprite_id));

        if has_special_requirements {
            // Skip rooms with restricted sprites (npcs and bosses)
            log::debug!(
                "UW ${:02X} -- found required  ${:02X}",
                uw_room_id as u8,
                header.spriteset_id as u8,
            );
            continue;
        }

        // Check if there is a key in the room.
        let has_key = uw_sprites.iter().any(|it| it.has_key());

        // Determine which placement strategy is required.
        let placement = match header.tag1.is_kill_room() || header.tag2.is_kill_room() {
            true => Placement::KillRoom,
            false => Placement::Room,
        };

        // Find all spritesets capable of switching to this spriteset.
        let compatible_spritesets = spritesets
            .iter()
            .filter(|spriteset| {
                is_list_compatible(
                    &existing_sprite_ids,
                    model.sprite_pool.get(&spriteset),
                    placement,
                    has_key,
                )
            })
            .map(|it| *it)
            .collect::<Vec<_>>();
        let maybe_matching_spriteset =
            compatible_spritesets.choose_weighted(rng, |spriteset_id| weights[spriteset_id]);

        // Switch spritesets. There should always be at least one match (the original spriteset).
        if let Ok(new_spriteset_id) = maybe_matching_spriteset.copied() {
            header.spriteset_id = new_spriteset_id;

            // Reduce the weight by half when selected.
            let weight = *weights.get(&new_spriteset_id).unwrap();
            let new_weight = max(WEIGHT_EMPTY, min(weight, weight / 2));
            weights.insert(new_spriteset_id, new_weight);
        } else {
            log::error!(
                "UW ${:02X} -- no valid swap for {:02X}",
                header.id as u8,
                header.spriteset_id as u8,
            );
        }
    }
}
