use std::collections::BTreeMap;

use binary_reader::BinaryReader;

use crate::zelda3::model::{Chest, HiddenItem, PaletteId, RoomBackground, RoomCollision, RoomEffect, RoomLogic, Secret, Sprite, SpriteId, SpritesetId, UWBlocksetId, UWFloorId, UWLayoutId, UWRoomId, UWScene, UWSpriteList, UnderworldRoomHeader};

use super::uw_scene_reader::bytes_to_scene;


/// Deserializes dungeon file data. This uses the same format as ZScream dungeon exporting.
pub(crate) fn read_dungeon_binary(byte: &[u8]) -> Result<ImportedDungeon, std::io::Error> {
    let mut binary_reader = BinaryReader::from_u8(byte);
    let room_count = binary_reader.read_i32()?;
    let mut imported_dungeon = ImportedDungeon::default();
    let mut pit_damage_rooms = vec![];

    for _ in 0..room_count {
        let room_id = UWRoomId::from_repr(binary_reader.read_i32()? as u16).expect("Invalid room id");
        let mut room = ImportedRoom::default();

        let object_byte_count = binary_reader.read_i32()? as usize;
        let scene_data = binary_reader.read(object_byte_count).expect("Decoding error");
        room.scene = bytes_to_scene(scene_data);

        let sprite_count = binary_reader.read_u8()?;
        
        for j in 0..sprite_count {
            let _roomindex = binary_reader.read_i16()?;
            let id_value = binary_reader.read_u8()?;
            let x = binary_reader.read_u8()?;
            let y = binary_reader.read_u8()?;
            let subtype = binary_reader.read_u8()?;
            let layer = binary_reader.read_u8()?;
            let id = SpriteId::from_repr(id_value as u16).expect("Sprite does not exist");

            if j > 0 && id == SpriteId::xE4_SMALL_KEY || id == SpriteId::xE5_BIG_KEY {
                // If this isn't the first item, assign big and small keys to the sprite before it.
                // Internally spritzer does not consider keys to be separate from their holders although
                // it is that way in both ZScream and the game.
                room.spritelist.sprites.last_mut().unwrap().item = Some(id);   
            } else {
                room.spritelist.sprites.push(Sprite {
                    id,
                    y,
                    x,
                    is_lower_layer: layer != 0,
                    item: None,
                    aux0: Some(subtype),
                    aux1: None,
                });
            }
        }

        let pot_count = binary_reader.read_u8()?;
        for j in 0..pot_count {
            let id = HiddenItem::from_repr(binary_reader.read_u8()?).expect("Invalid hidden item");
            let x = binary_reader.read_u8()?;
            let y = binary_reader.read_u8()?;
            let is_lower_layer = binary_reader.read_bool()?;
            room.secrets.push(Secret {
                x,
                y,
                is_lower_layer,
                item: Some(id),
            });
        }

        let chest_count = binary_reader.read_u8()?;
        for _ in 0..chest_count {
            let x = binary_reader.read_u8()?;
            let y = binary_reader.read_u8()?;
            let item = binary_reader.read_u8()?;
            let big_chest = binary_reader.read_bool()?;
            room.chests.push(Chest{
                x, y, item, big_chest
            });
        }

        for _ in 0..0x1000 {
            let _collision_map = binary_reader.read_u8()?;
        }

        let has_pit_damage = binary_reader.read_bool()?;
        if has_pit_damage {
            pit_damage_rooms.push(room_id);
        }

        room.header.holewarp = UWRoomId::from_repr(binary_reader.read_u8()? as u16).expect("Invalid room id");
        room.header.stairs1 = UWRoomId::from_repr(binary_reader.read_u8()? as u16).expect("Invalid room id");
        room.header.stairs2 = UWRoomId::from_repr(binary_reader.read_u8()? as u16).expect("Invalid room id");
        room.header.stairs3 = UWRoomId::from_repr(binary_reader.read_u8()? as u16).expect("Invalid room id");
        room.header.stairs3 = UWRoomId::from_repr(binary_reader.read_u8()? as u16).expect("Invalid room id");
        room.header.holewarp_plane = binary_reader.read_u8()?;
        room.header.stairs1_plane = binary_reader.read_u8()?;
        room.header.stairs2_plane = binary_reader.read_u8()?;
        room.header.stairs3_plane = binary_reader.read_u8()?;
        room.header.stairs4_plane = binary_reader.read_u8()?;
        room.header.tag1 = RoomLogic::from_repr(binary_reader.read_u8()?).expect("Invalid room tag");
        room.header.tag2 = RoomLogic::from_repr(binary_reader.read_u8()?).expect("Invalid room tag");
        room.header.bg2 = RoomBackground::from_repr(binary_reader.read_u8()?).expect("Invalid room background");
        room.header.spriteset_id = SpritesetId::from_repr(binary_reader.read_u8()?).expect("Invalid spriteset id");

        room.spritelist.sorted = binary_reader.read_bool()?;
        room.header.palette_id = PaletteId::from_repr(binary_reader.read_u8()?).expect("Invalid palette id");
        let _messageid = binary_reader.read_u8()?;

        room.header.effect = RoomEffect::from_repr(binary_reader.read_u8()?).expect("Invalid effect");
        room.scene.layout.floor1 = UWFloorId::from_repr(binary_reader.read_u8()?).expect("Invalid floor id");
        room.scene.layout.floor2 = UWFloorId::from_repr(binary_reader.read_u8()?).expect("Invalid floor id");
        room.header.collision = RoomCollision::from_repr(binary_reader.read_u8()?).expect("Invalid collision");
        room.header.light = binary_reader.read_bool()?;
        room.header.blockset_id =  UWBlocksetId::from_repr(binary_reader.read_u8()?).expect("Invalid Blockset Id");
        room.scene.layout.layout = UWLayoutId::from_repr(binary_reader.read_u8()?).expect("Invalid layout id");
        imported_dungeon.rooms.insert(room_id, room);
    }

    imported_dungeon.pit_damage = pit_damage_rooms;
    
    Ok(imported_dungeon)
}

pub(crate) struct ImportedDungeon {
    rooms: BTreeMap<UWRoomId, ImportedRoom>,
    pit_damage: Vec<UWRoomId>,
}

impl ImportedDungeon {
    pub(crate) fn default() -> ImportedDungeon {
        ImportedDungeon {
            rooms: BTreeMap::default(),
            pit_damage: vec![],
        }
    }
}

pub(crate) struct ImportedRoom {
    header: UnderworldRoomHeader,
    chests: Vec<Chest>,
    scene: UWScene,
    secrets: Vec<Secret>,
    spritelist: UWSpriteList,
}

impl ImportedRoom {
    pub(crate) fn default() -> ImportedRoom {
        ImportedRoom {
            chests: vec![],
            header: UnderworldRoomHeader::default(),
            scene: UWScene::default(),
            secrets: vec![],
            spritelist: UWSpriteList::default(),
        }
    }
}