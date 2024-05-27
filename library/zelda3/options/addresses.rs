use assembly::zelda3::SettingAddress;

use crate::zelda3::GameVersion;

pub(crate) struct Addresses {
    pub(crate) bush_secret_ptrs: usize,
    pub(crate) damage_class: usize,
    pub(crate) damage_subclass: usize,
    pub(crate) door_ptrs: usize,
    pub(crate) enable_killable_thief: usize,
    pub(crate) entrances: usize,
    pub(crate) is_hera_prize_centered: usize,
    pub(crate) layout_ptrs: usize,
    pub(crate) moldorm_eye_count: usize,
    pub(crate) owspecial_graphics: usize,
    pub(crate) owspecial_palette: usize,
    pub(crate) owsprite_ptrs: usize,
    pub(crate) owroom_empty: usize,
    pub(crate) pit_damage: usize,
    pub(crate) pot_secret_ptrs: usize,
    pub(crate) room_data_sprite_pointers_ref0: usize,
    pub(crate) room_sprites_end: usize,
    pub(crate) spriteset: usize,
    pub(crate) sprite_settings: usize,
    pub(crate) uwgraphics: usize,
    pub(crate) uwheader_bank: usize,
    pub(crate) uwheader_ref0: usize,
    pub(crate) uwsprite_ptrs: usize,
}

impl Addresses {
    pub(crate) fn for_version(version: GameVersion) -> Addresses {
        match version {
            GameVersion::UNKNOWN => get_jp_addresses(),
            GameVersion::ZeldaJp => get_jp_addresses(),
            GameVersion::ZeldaUs => get_jp_addresses(),
            GameVersion::Archipelago => get_jp_addresses(),
            GameVersion::Alttpr => get_jp_addresses(),
            GameVersion::ArchipelagoEnemizer => get_jp_addresses(),
            GameVersion::AlttprEnemizer => get_jp_addresses(),
        }
    }
}

fn get_jp_addresses() -> Addresses {
    Addresses {
        enable_killable_thief: SettingAddress::EnableKillableThief.into(),
        is_hera_prize_centered: SettingAddress::IsHeraPrizeCentered.into(),
        moldorm_eye_count: SettingAddress::MoldormEyeCount.into(),
        bush_secret_ptrs: 0x1BC8B9,
        damage_class: 0x06F42D,
        damage_subclass: 0x0DB8F1,
        door_ptrs: 0x1F83C0,
        entrances: 0x02C577,
        layout_ptrs: 0x1F8000,
        owspecial_graphics: 0x02E575,
        owspecial_palette: 0x02E596,
        owsprite_ptrs: 0x09C881,
        owroom_empty: 0x9CB41,
        pit_damage: 0x0794A2,
        pot_secret_ptrs: 0x01E6C0,
        room_data_sprite_pointers_ref0: 0x09C298,
        room_sprites_end: 0x09EC9C,
        spriteset: 0x00DB97,
        sprite_settings: 0x0DB080,
        uwgraphics: 0x00FA41,
        uwheader_bank: 0x01B5E7,
        uwheader_ref0: 0x01B5DD,
        uwsprite_ptrs: 0x09D62E,
    }
}
