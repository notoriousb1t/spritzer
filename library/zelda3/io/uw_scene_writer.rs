use assembly::zelda3::Symbol;
use std::collections::hash_map::Entry;
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::common::readerwriter::WriteObject;
use crate::snes::SnesGame;
use crate::zelda3::model::UWDoorList;
use crate::zelda3::model::UWLayout;
use crate::zelda3::model::UWLayoutId;
use crate::zelda3::model::UWObject;
use crate::zelda3::model::UWRoomId;
use crate::zelda3::model::UWScene;

const STOP_MARKER: u8 = 0xFF;
const LAYER_MARKER: u8 = 0xFF;
const END_MARKER: u8 = 0xF0;

impl WriteObject<BTreeMap<UWRoomId, UWScene>> for SnesGame {
    fn write_objects(&mut self, scenes: &BTreeMap<UWRoomId, UWScene>) {
        // Group room ids that have exact layouts, objects, and entrances.
        let mut map: HashMap<&UWScene, Vec<UWRoomId>> = HashMap::new();
        let mut scenes_tuples = scenes.iter().collect::<Vec<_>>();
        scenes_tuples.sort_by_key(|it| it.0);

        for (id, scene) in scenes_tuples {
            let values = match map.entry(scene) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(vec![]),
            };
            values.push(*id);
        }

        // Sort groups by the first room that occurs in them. This must be stable, and this is good enough.
        // Without this, the seed string isn't stable and it is hard to pinpoint issues or have two players
        // play the same game.
        let mut groups = map.iter().collect::<Vec<_>>();
        groups.sort_by_key(|it| it.1[0]);

        // Write layouts and pointers for each layout and entrance configuration.
        // Layouts and doors, despite having separate pointer tables must be exactly sequential to
        // each other. That is, each layout must be followed by the correct door
        // configuration. If this is not done sequentially, it introduces hard to find gamebreaking
        // bugs.
        for (scene, ids) in groups {
            let layout_bytes = layout_to_bytes(&scene.layout);
            let entrance_bytes = doorlist_to_bytes(&scene.doors);
            let scene_bytes = layout_bytes
                .iter()
                .chain(entrance_bytes.iter())
                .cloned()
                .collect::<Vec<_>>();

            if let Some(bank) = self.find_capacity(scene_bytes.len()) {
                // Write the bytes all at once because multiple freespace objects may exist in the
                // same bank. Writing both at once guarantees contiguous record.
                let layout_location = self.write_data(&[bank], &scene_bytes).unwrap();
                // The entrance location should follow the layout information.
                let entrance_location = layout_location + layout_bytes.len();

                for id in ids.iter() {
                    self.write_pointer(
                        Symbol::LayoutPtrs as usize + (*id as usize * 3),
                        layout_location,
                    );
                    self.write_pointer(
                        Symbol::DoorPtrs as usize + (*id as usize * 3),
                        entrance_location,
                    );
                }
            } else {
                panic!("No space writing layout/entrance for {}", ids[0]);
            }
        }
    }
}

/// Write the doors and return the new location of the cursor.
fn doorlist_to_bytes(doors: &UWDoorList) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    for door in doors.iter() {
        bytes.push(((door.position as u8) << 3) | (door.direction as u8));
        bytes.push(door.style as u8);
    }

    // Write end of doors marker.
    bytes.push(STOP_MARKER);
    bytes.push(STOP_MARKER);
    bytes
}

fn layout_to_bytes(layout: &UWLayout) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    // Write floors and layout back to the general layout.
    bytes.extend(layout_preamble_to_bytes(
        layout.floor1,
        layout.floor2,
        layout.layout,
        layout.aux0,
    ));

    for (layer_index, layer) in layout.layers.iter().enumerate() {
        if layer_index != 0 {
            bytes.push(LAYER_MARKER);
            bytes.push(LAYER_MARKER);
        }

        for object in layer.iter() {
            bytes.extend(&layout_object_to_bytes(object));
        }
    }

    // Write end of doors marker.
    bytes.push(END_MARKER);
    bytes.push(LAYER_MARKER);
    bytes
}

/// Converts an underworld room object back to its corresponding bytes.
fn layout_object_to_bytes(object: &UWObject) -> [u8; 3] {
    if object.id as u16 >= 0x200 {
        // Subtype 3
        return [
            (((object.x as u16) << 2) | (object.id as u16 & 0x3)) as u8,
            (((object.y as u16) << 2) | ((object.id as u16 & 0xC) >> 2)) as u8,
            (0xF8 | (((object.id as u16) - 0x200) >> 4)) as u8,
        ];
    }
    if object.id as u16 >= 0x100 {
        // Subtype 2
        return [
            0xFC | (object.x >> 4),
            (object.x << 4) | ((object.y & 0x3C) >> 2),
            (object.y << 6) | (object.id as u16 & 0x3F) as u8,
        ];
    }
    // Subtype 1
    [
        (object.x << 2) | object.width,
        (object.y << 2) | object.height,
        object.id as u8,
    ]
}

fn layout_preamble_to_bytes(floor1: u8, floor2: u8, layout: UWLayoutId, aux0: u8) -> [u8; 2] {
    [(floor2 << 4 | floor1), (layout as u8) << 2 | aux0]
}

#[cfg(test)]
mod tests {
    use crate::zelda3::io::uw_scene_writer::layout_object_to_bytes;
    use crate::zelda3::io::uw_scene_writer::layout_preamble_to_bytes;
    use crate::zelda3::model::UWLayoutId;
    use crate::zelda3::model::UWObject;

    #[test]
    fn converts_preamble_to_bytes() {
        let actual = layout_preamble_to_bytes(0x1, 0x2, UWLayoutId::X6_BottomSplit, 0x3);
        let expected = [0x21, 0x1B];
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_object_to_bytes_subtype1() {
        let object = UWObject::from_value(0xC4, 0x31, 0x05, 2, 3);
        let actual = layout_object_to_bytes(&object);
        let expected = [0xC6, 0x17, 0xC4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_object_to_bytes_subtype2() {
        let object = UWObject::from_value(0x120, 0x2F, 0x2A, 0, 0);
        let actual = layout_object_to_bytes(&object);
        let expected = [0xFE, 0xFA, 0xA0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_object_to_bytes_subtype3() {
        let object = UWObject::from_value(0x277, 0x20, 0x04, 0, 0);
        let actual = layout_object_to_bytes(&object);
        let expected = [0x83, 0x11, 0xFF];
        assert_eq!(actual, expected);
    }
}
