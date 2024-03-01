use log::debug;

use crate::common::readerwriter::WriteObject;
use crate::snes::SnesGame;
use crate::zelda3::model::Z3Model;

impl WriteObject<Z3Model> for SnesGame {
    fn write_objects(&mut self, model: &Z3Model) {
        // Clear all known freespace and fill with 0s.
        self.deallocate();

        // Write all the standard objects back. This needs to happen afterward in case
        // a direct write needs to modify an object before it is written.

        // Note: each of these types as a corresponding writer suffixed module
        // that implements write_objects() for the inferred type.
        self.write_objects(&model.damage_subclasses);
        self.write_objects(&model.damage_classes);
        self.write_objects(&model.sprite_settings);
        self.write_objects(&model.spritesets);

        self.write_objects(&model.ow_rooms);
        self.write_objects(&model.uw_headers);
        self.write_objects(&model.uw_sprites);
        self.write_objects(&model.uw_scenes);
        self.write_objects(&model.uw_entrances);

        debug!("Capacity");
        for space in self.free_space.iter() {
            debug!("    {:02X} has {} bytes", space.bank, space.capacity());
        }
    }
}
