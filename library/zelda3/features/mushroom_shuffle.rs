use rand::seq::SliceRandom;

use crate::zelda3::model::OWRoomId;
use crate::zelda3::model::SpriteId;
use crate::zelda3::model::Z3Model;

pub(crate) fn apply_mushroom_shuffle(model: &mut Z3Model) {
    let mut rng = model.create_rng();

    // Find lost woods.
    let lost_woods_lw = model
        .ow_rooms
        .get_mut(&OWRoomId::x0_LOST_WOODS)
        .expect(&format!("Missing Lost Woods"));

    // Change the mushroom to be a fake sword.
    if let Some(mushroom) = lost_woods_lw
        .lw
        .sprites
        .iter_mut()
        .find(|it| it.id == SpriteId::xE7_MUSHROOM)
    {
        mushroom.id = SpriteId::xE8_FAKE_MASTER_SWORD;
    }

    if let Some(fake_sword) = lost_woods_lw
        .lw
        .sprites
        .iter_mut()
        .filter(|it| it.id == SpriteId::xE8_FAKE_MASTER_SWORD)
        .collect::<Vec<_>>()
        .choose_mut(&mut rng)
    {
        // Randomly assign fake master sword as mushroom.
        fake_sword.id = SpriteId::xE7_MUSHROOM;
    }
}
