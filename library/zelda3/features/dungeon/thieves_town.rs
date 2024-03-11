use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::Z3Model;

pub(crate) fn prepare_thieves_town_boss(model: &mut Z3Model) {
    let scene = model
        .uw_scenes
        .get_mut(&UWRoomId::xAC_THIEVES_TOWN_BLIND_THE_THIEF_BOSS)
        .unwrap();
    for layer in scene.layout.layers.iter_mut() {
        layer.clear();
    }
}
