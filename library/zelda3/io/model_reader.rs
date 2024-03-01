use crate::common::readerwriter::ReadObject;
use crate::snes::SnesGame;
use crate::zelda3::model::Z3Model;

impl ReadObject<Z3Model> for SnesGame {
    fn read_objects(&self) -> Z3Model {
        // Create a new model and read all data types to their respective names.
        let mut model = Z3Model::default();
        // Note: each of these types as a corresponding reader suffixed module
        // that implements read_objects() for the inferred type.
        model.damage_classes = self.read_objects();
        model.damage_subclasses = self.read_objects();
        model.ow_rooms = self.read_objects();
        model.sprite_settings = self.read_objects();
        model.spritesets = self.read_objects();
        model.dungeons = self.read_objects();
        model.uw_scenes = self.read_objects();
        model.uw_headers = self.read_objects();
        model.uw_sprites = self.read_objects();
        model.uw_entrances = self.read_objects();
        model
    }
}
