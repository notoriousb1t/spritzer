use super::ow_room_id::OWRoomId;
use super::ow_state::OWStateId;
use super::sprite::Sprite;
use super::spriteset_id::SpritesetId;

#[derive(Clone)]
pub(crate) struct OWRoom {
    /// The Area this block describes. DO NOT MODIFY.
    pub id: OWRoomId,
    /// This is generally the room before Agahnim is defeated.
    pub lw: OWRoomState,
    /// The version to use if pre-rescue is relevant. (Link's House, Hyrule Castle, and forest west
    /// of Link's House)
    pub lw_rescue: Option<OWRoomState>,
    /// The version to use if post-aga is relevant. (Lumberjack and Hyrule Castle)
    pub lw_post_aga: Option<OWRoomState>,
    // This is used in the dark world.
    pub dw: Option<OWRoomState>,
}

#[derive(Clone)]
pub(crate) struct OWRoomState {
    /// The overworld state this room is for.
    pub overworld_id: OWStateId,
    /// The graphics block associated. This constrains which Entities can appear in this Area.
    pub spriteset_id: SpritesetId,
    /// Sprite Palette ID.
    pub sprite_palette_id: u8,
    /// The Entities in this Overworld Area.
    pub sprites: Vec<Sprite>,
}

impl OWRoom {
    pub fn states(&self) -> Vec<&OWRoomState> {
        let mut configs: Vec<&OWRoomState> = vec![];
        configs.push(&self.lw);
        if let Some(it) = &self.lw_rescue {
            configs.push(it);
        }
        if let Some(it) = &self.lw_post_aga {
            configs.push(it);
        }
        if let Some(it) = &self.dw {
            configs.push(it);
        }
        configs
    }

    pub fn versions(&self) -> Vec<&OWRoomState> {
        let mut configs: Vec<&OWRoomState> = vec![];
        configs.push(&self.lw);
        if let Some(it) = &self.lw_rescue {
            configs.push(it);
        }
        if let Some(it) = &self.lw_post_aga {
            configs.push(it);
        }
        if let Some(it) = &self.dw {
            configs.push(it);
        }
        configs
    }

    pub fn get_mut(&mut self, state_id: OWStateId) -> Option<&mut OWRoomState> {
        match state_id {
            OWStateId::LIGHT_WORLD_V0 => self.lw_rescue.as_mut(),
            OWStateId::LIGHT_WORLD_V1 => Some(&mut self.lw),
            OWStateId::DARK_WORLD_V1 => self.dw.as_mut(),
            OWStateId::LIGHT_WORLD_V2 => self.lw_post_aga.as_mut(),
            OWStateId::DARK_WORLD_V2 => self.dw.as_mut(),
        }
    }

    pub fn versions_mut(&mut self) -> Vec<&mut OWRoomState> {
        let mut configs: Vec<&mut OWRoomState> = vec![];
        configs.push(&mut self.lw);
        if let Some(it) = &mut self.lw_rescue {
            configs.push(it);
        }
        if let Some(it) = &mut self.lw_post_aga {
            configs.push(it);
        }
        if let Some(it) = &mut self.dw {
            configs.push(it);
        }
        configs
    }

    pub fn sprites_mut(&mut self) -> Vec<&mut Sprite> {
        let mut sprites: Vec<&mut Sprite> = vec![];
        for version in self.versions_mut() {
            for sprite in &mut version.sprites {
                sprites.push(sprite);
            }
        }
        sprites
    }
}
