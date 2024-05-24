use rand::prelude::SliceRandom;
use strum::IntoEnumIterator;

use crate::zelda3::model::{PotSecret, Quadrant, Secret, UWLayout, UWObjectId, UWRoomId, Z3Model};

pub(super) fn add_pot_tricks(_model: &mut Z3Model) {}

pub(super) fn shuffle_pot_secrets(model: &mut Z3Model) {
    let mut rng = model.create_rng();

    for room_id in UWRoomId::iter() {
        // Get the scene and secrets which have to be merged to do the shuffle.
        let scene = model.uw_scenes.get(&room_id).expect("UWScene is missing");
        let secrets = model
            .uw_pot_secrets
            .get(&room_id)
            .expect("UWRoom pot secrets are missing");
        // Get all pots empty or not.
        let all_pots = get_all_pots(&scene.layout, &secrets);

        // At most there are 4 areas possible in a room, so iterate over that number. This means
        // some of the loops are operating on empty collections. (which is fine)
        let mut results: Vec<PotSecret> = vec![];
        for group_number in 0..4usize {
            // Inside of the group, convert back to a pot secret and only operate on this group number.
            let mut pot_group: Vec<PotSecret> = all_pots
                .iter()
                .filter(|pot| pot.group == group_number)
                .map(|pot| PotSecret {
                    x: pot.x,
                    y: pot.y,
                    z: pot.z,
                    secret: pot.secret,
                })
                .collect();
            // Create a copy of all secrets in the same order as the group.
            let mut shuffled_secrets: Vec<Option<Secret>> =
                pot_group.iter().map(|pot| pot.secret).collect();
            // Shuffle the secrets.
            shuffled_secrets.shuffle(&mut rng);
            // Reassign the secret to the pot which in most cases, creates an empty pot.
            for (i, secret) in shuffled_secrets.iter().enumerate() {
                pot_group[i].secret = *secret;
            }
            // Add only the pots that have secrets to the results.
            results.extend(pot_group.iter().filter(|it| it.secret.is_some()));
        }

        if results.len() > secrets.len() {
            panic!(
                "Pot Shuffle failed due to a programming error:\nResults = {:#?}\nSecrets={:#?}",
                results,
                secrets
            );
        }

        // Replace the pot secrets for this room.
        model.uw_pot_secrets.insert(room_id, results);
    }
}

fn get_all_pots(layout: &UWLayout, secrets: &Vec<PotSecret>) -> Vec<Pot> {
    // Get the layout groups for this layout. This is used to group each pot secret
    // so they can be shuffled in groups unlikely to break logic.
    let layout_groups = layout.layout.quadrant_groups();

    layout
        .layers
        .iter()
        .enumerate()
        .flat_map(|(index, layer)| {
            layer
                .iter()
                .filter(|it| match it.id {
                    // Use a match in case there are other containers. I don't think 
                    UWObjectId::X22F_Pot => true,
                    _ => false,
                })
                .map(|obj| {
                    let quadrant = Quadrant::from_point(obj.x, obj.y, 0x40).unwrap();
                    let group = layout_groups
                        .iter()
                        .enumerate()
                        .filter(|(_, quadrants)| quadrants.contains(&quadrant))
                        .map(|(i, _)| i)
                        .next()
                        .unwrap();
                    let secret = secrets
                        .iter()
                        .filter(|it| it.secret.is_some() && obj.x == it.x && obj.y == it.y)
                        .map(|it| it.secret.unwrap())
                        .next();
                    Pot {
                        x: obj.x,
                        y: obj.y,
                        z: index > 0,
                        group,
                        secret,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

struct Pot {
    x: u8,
    y: u8,
    z: bool,
    secret: Option<Secret>,
    group: usize,
}
