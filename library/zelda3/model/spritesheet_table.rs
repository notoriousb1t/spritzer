use super::{SpriteId, SpritesheetId};

/// Evaluates an array of spritesheets and list of sprite_ids and determines which Spritesheets are
/// required to render that list of sprites.
pub(crate) fn get_sprite_requirements(
    spriteset: [SpritesheetId; 4],
    sprite_ids: &[SpriteId],
    is_underworld: bool,
) -> [SpritesheetId; 4] {
    let mut spriteset_result = [SpritesheetId::None; 4];

    // Fill with all required spritesheets.
    if is_underworld {
        for i in 0..4 {
            if is_spritesheet_permanent_uw(&spriteset[i]) {
                spriteset_result[i] = spriteset[i];
            }
        }
    }

    // Gather a list of all known spritesheet arrangements for the required sprites.
    let sprite_requirements = sprite_ids
        .iter()
        .flat_map(|sprite_id| get_spritesheet_arrangements(sprite_id));

    // Walk through each requirement and set it if the original spriteset has it.
    for requirement in sprite_requirements {
        for i in 0..4 {
            if requirement[i] != SpritesheetId::None && requirement[i] == spriteset[i] {
                spriteset_result[i] = spriteset[i];
            }
        }
    }

    spriteset_result
}

/// True if the spritesheet cannot be reassigned under any circumstance.
pub(crate) fn is_spritesheet_permanent_uw(spritesheet_id: &SpritesheetId) -> bool {
    match spritesheet_id {
        // Although uncle and priest are on the same sprite, they don't use
        // the same offsets, so the sprite graphics are not interchangeable.
        SpritesheetId::x47_PRIEST => true,
        SpritesheetId::x51_UNCLE_PRIEST_SICK_BOY => true,
        // Somaria platforms are spawned in, so it is impossible to tell if a room has this without
        // checking tile data.
        SpritesheetId::x27_TURTLE_ROCK => true,
        _ => false,
    }
}

/// Determines which possible arrangements of Spritesheets are needed to render the given sprite id.
pub(crate) fn get_spritesheet_arrangements(sprite_id: &SpriteId) -> Vec<[SpritesheetId; 4]> {
    match sprite_id {
        SpriteId::x0_RAVEN => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x19_SWAMOLA_CROW,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x11_MISC_FAKE_SWORD,
            ],
        ],
        SpriteId::x1_VULTURE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x12_DESERT_1,
            SpritesheetId::None,
        ]],
        SpriteId::x2_STALFOS_HEAD => vec![[
            SpritesheetId::x1F_STALFOS_BARI,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        // This should never match anything. It will break the game, seriously!
        SpriteId::x3_NONE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x4_PULL_SWITCH_NORMAL => vec![
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x55_AgahnimCrystalMaiden,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x5_PULL_SWITCH_NORMAL_UNUSED => vec![
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x55_AgahnimCrystalMaiden,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x6_PULL_SWITCH_TRAP => vec![
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x55_AgahnimCrystalMaiden,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x7_PULL_SWITCH_TRAP_UNUSED => vec![
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x55_AgahnimCrystalMaiden,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x8_OCTOROK => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::xC_OCTOROK_ZORA,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x18_OCTOROCKS,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x9_MOLDORM => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x30_MOLDORM_BOSS,
            SpritesheetId::None,
        ]],
        SpriteId::xA_OCTOROK_FOUR_WAY => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::xC_OCTOROK_ZORA,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x18_OCTOROCKS,
                SpritesheetId::None,
            ],
        ],
        SpriteId::xB_CUCCO => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::xE_POE_THIEF_LW,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x15_POE_THIEF_DW,
            ],
        ],
        SpriteId::xC_OCTOROK_STONE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x18_OCTOROCKS,
            SpritesheetId::None,
        ]],
        SpriteId::xD_BUZZBLOB => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x11_MISC_FAKE_SWORD,
        ]],
        SpriteId::xE_SNAPDRAGON => vec![[
            SpritesheetId::x16_HINOX_SNAPDRAGON,
            SpritesheetId::None,
            SpritesheetId::x17_MOBLIN,
            SpritesheetId::None,
        ]],
        SpriteId::xF_OCTOBALLOON => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::xC_OCTOROK_ZORA,
            SpritesheetId::None,
        ]],
        SpriteId::x10_OCTOBALLOON_HATCHLINGS => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::xC_OCTOROK_ZORA,
            SpritesheetId::None,
        ]],
        SpriteId::x11_HINOX => vec![[
            SpritesheetId::x16_HINOX_SNAPDRAGON,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x12_MOBLIN => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x17_MOBLIN,
            SpritesheetId::None,
        ]],
        SpriteId::x13_MINI_HELMASAUR => vec![[
            SpritesheetId::None,
            SpritesheetId::x1E_MINI_MONSTERS,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        // SpriteId::x14_THIEVES_TOWN_GRATE
        SpriteId::x15_ANTIFAIRY => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x53_ObjectsHazards,
        ]],
        SpriteId::x16_SAHASRAHLA => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4C_SAHASRAHLA_WITCH,
            SpritesheetId::None,
        ]],
        SpriteId::x17_HOARDER => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x11_MISC_FAKE_SWORD,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x10_MISC_ROCKS,
            ],
        ],
        SpriteId::x18_MINI_MOLDORM => vec![[
            SpritesheetId::None,
            SpritesheetId::x1E_MINI_MONSTERS,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x19_POE => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::xE_POE_THIEF_LW,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x15_POE_THIEF_DW,
            ],
            // Homebrew (floating hoarder 1)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x10_MISC_ROCKS,
            ],
            // Homebrew (floating hoarder 2)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x11_MISC_FAKE_SWORD,
            ],
            // Homebrew (floating zirro)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x1B_MISCELLANEOUS_DW_1,
            ],
            // Homebrew (floating skull)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x20_STALFOS_KNIGHT_VERMIN,
            ],
            // Homebrew (floating orb)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x25_WIZZROBE_SLUGGULA,
            ],
            // Homebrew (floating orb)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x26_FROSTY_FRIENDS,
            ],
            // Homebrew (floating orb)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x36_MASTER_SWORD,
            ],
            // Homebrew (floating orb)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x39_ARRGHUS_BOSS,
            ],
            // Homebrew (floating eye)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x3D_VITREOUS_BOSS,
            ],
            // Homebrew (floating eye)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x41_BIG_BAD_GUY,
            ],
            // Homebrew (floating rabbit)
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
            ],
        ],
        SpriteId::x1A_SMITHY => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4A_KAKARIKO,
            SpritesheetId::None,
        ]],
        // SpriteId::x1B_ARROW
        SpriteId::x1C_STATUE => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x52_ObjectsHazards,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x53_ObjectsHazards,
            ],
        ],
        SpriteId::x1D_FLUTEQUEST => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
            SpritesheetId::None,
        ]],
        SpriteId::x1E_CRYSTAL_SWITCH => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x52_ObjectsHazards,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x53_ObjectsHazards,
            ],
        ],
        SpriteId::x1F_BUG_CATCHING_KID => vec![[
            SpritesheetId::x51_UNCLE_PRIEST_SICK_BOY,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x20_SLUGGULA => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x25_WIZZROBE_SLUGGULA,
            SpritesheetId::None,
        ]],
        SpriteId::x21_WATER_SWITCH => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x53_ObjectsHazards,
        ]],
        SpriteId::x22_ROPA => vec![[
            SpritesheetId::x16_HINOX_SNAPDRAGON,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x23_RED_BARI => vec![[
            SpritesheetId::x1F_STALFOS_BARI,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x24_BLUE_BARI => vec![[
            SpritesheetId::x1F_STALFOS_BARI,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x25_TALKING_TREE => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x1B_MISCELLANEOUS_DW_1,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x19_SWAMOLA_CROW,
            ],
        ],
        SpriteId::x26_HARDHAT_BEETLE => vec![[
            SpritesheetId::None,
            SpritesheetId::x1E_MINI_MONSTERS,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x27_DEADROCK => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::x28_HINT_PC_DW => vec![[
            SpritesheetId::x4B_ARCHERY,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x29_BLIND_HIDEOUT_ATTENDANT => vec![[
            SpritesheetId::x4F_OLD_MAN_RUNNER,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x2A_SWEEPING_LADY => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4A_KAKARIKO,
            SpritesheetId::None,
        ]],
        SpriteId::x2B_TENTMAN => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x37_MASTER_SWORD,
            SpritesheetId::None,
        ]],
        SpriteId::x2C_LUMBERJACKS => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4A_KAKARIKO,
            SpritesheetId::None,
        ]],
        SpriteId::x2E_FLUTE_KID => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x4C_SAHASRAHLA_WITCH,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x2F_RACE_LADY => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x50_CUCCO_FOR_NPCS,
        ]],
        SpriteId::x30_RACE_GUY => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x50_CUCCO_FOR_NPCS,
        ]],
        SpriteId::x31_FORTUNE_TELLER => vec![[
            SpritesheetId::x4B_ARCHERY,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x32_ANGRY_BROTHERS => vec![[
            SpritesheetId::x4F_OLD_MAN_RUNNER,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        // SpriteId::x33_RUPEE_PULL
        SpriteId::x34_SNITCH_YOUNG => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x50_CUCCO_FOR_NPCS,
        ]],
        // SpriteId::x35_INNKEEPER
        SpriteId::x36_WITCH => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4C_SAHASRAHLA_WITCH,
            SpritesheetId::None,
        ]],
        // SpriteId::x37_WATERFALL
        // SpriteId::x38_EYEGORE_STATUE
        SpriteId::x39_LOCKSMITH => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x12_DESERT_1,
            SpritesheetId::x11_MISC_FAKE_SWORD,
        ]],
        SpriteId::x3A_MAGIC_BAT => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x1D_ARMOS_BOSS_LOCK_BAT,
        ]],
        SpriteId::x3B_BONK_ITEM => vec![[
            SpritesheetId::xF_DASH_HOARDER,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x3C_VILLAGE_KID => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4A_KAKARIKO,
            SpritesheetId::None,
        ]],
        SpriteId::x3D_SNITCH_OLD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x50_CUCCO_FOR_NPCS,
        ]],
        SpriteId::x3E_HOARDER_ROCK => vec![
            [
                SpritesheetId::xF_DASH_HOARDER,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x11_MISC_FAKE_SWORD,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x10_MISC_ROCKS,
            ],
        ],
        SpriteId::x3F_TUTORIAL_SOLDIER => vec![[
            SpritesheetId::x48_SOLDIER,
            SpritesheetId::x49_SOLDIERS,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x40_LIGHTNING_LOCK => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x1C_PESTS,
            SpritesheetId::x1D_ARMOS_BOSS_LOCK_BAT,
        ]],
        SpriteId::x41BlueSwordGuard => vec![
            [
                SpritesheetId::None,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x42_GREEN_GUARD => vec![
            [
                SpritesheetId::None,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x43_RED_SPEAR_GUARD => vec![
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                // Homebrew: Borrow the top half from lightworld.
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x44_BLUE_ASSAULT_GUARD => vec![
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x45RedSpearGuard2 => vec![
            [
                SpritesheetId::None,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x46_BLUE_ARCHER => vec![
            [
                SpritesheetId::x48_SOLDIER,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x48_SOLDIER,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x47_GREEN_GUARD_BUSH => vec![
            [
                SpritesheetId::x48_SOLDIER,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x48_SOLDIER,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x48_RED_JAVELIN_GUARD => vec![[
            SpritesheetId::x46_SOLDIERS,
            SpritesheetId::x49_SOLDIERS,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x49_RED_GUARD_BUSH => vec![
            // Vanilla.
            [
                SpritesheetId::x48_SOLDIER,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x48_SOLDIER,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x4A_RED_BOMB_GUARD => vec![
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x4B_GREEN_KNIFE_GUARD => vec![
            // Vanilla.
            [
                SpritesheetId::None,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::x13_SOLDIER_RECRUITS,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::x13_SOLDIER_RECRUITS,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::x13_SOLDIER_RECRUITS,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x4C_GELDMAN => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x12_DESERT_1,
            SpritesheetId::None,
        ]],
        SpriteId::x4D_TOPPO => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x11_MISC_FAKE_SWORD,
        ]],
        SpriteId::x4E_POPO_1 => vec![[
            SpritesheetId::None,
            SpritesheetId::x2C_BEAM_ME_UP_MR_POPO,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x4F_POPO_2 => vec![[
            SpritesheetId::None,
            SpritesheetId::x2C_BEAM_ME_UP_MR_POPO,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x50_CANNON_BALL => vec![[
            SpritesheetId::None,
            SpritesheetId::x2E_EyeGoreCannonBall,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x51_ARMOS_STATUE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::x52_ZORA_KING => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x44_ZORAS_DOMAIN,
        ]],
        SpriteId::x53_ARMOS_KNIGHT => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x1D_ARMOS_BOSS_LOCK_BAT,
        ]],
        SpriteId::x54_LANMOLAS => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x31_LANMOLAS_BOSS,
        ]],
        SpriteId::x55_FIREBALL_ZORA => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x18_OCTOROCKS,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::xC_OCTOROK_ZORA,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x56_WALKING_ZORA => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::xC_OCTOROK_ZORA, // Verify?
            SpritesheetId::x44_ZORAS_DOMAIN,
        ]],
        SpriteId::x57_DESERT_STATUE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x12_DESERT_1,
            SpritesheetId::None,
        ]],
        SpriteId::x58_CRAB => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::xC_OCTOROK_ZORA,
            SpritesheetId::None,
        ]],
        SpriteId::x59_LOST_WOODS_BIRD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x36_MASTER_SWORD,
        ]],
        SpriteId::x5A_LOST_WOODS_SQUIRREL => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x36_MASTER_SWORD,
        ]],
        SpriteId::x5B_SPARK_CLOCKWISE => vec![[
            SpritesheetId::x1F_STALFOS_BARI,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x5C_SPARK_COUNTER_CLOCKWISE => vec![[
            SpritesheetId::x1F_STALFOS_BARI,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x5D_ROLLER_SOUTH => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x27_TURTLE_ROCK,
            SpritesheetId::None,
        ]],
        SpriteId::x5E_ROLLER_NORTH => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x27_TURTLE_ROCK,
            SpritesheetId::None,
        ]],
        SpriteId::x5F_ROLLER_EAST => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x27_TURTLE_ROCK,
            SpritesheetId::None,
        ]],
        SpriteId::x60_ROLLER_WEST => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x27_TURTLE_ROCK,
            SpritesheetId::None,
        ]],
        SpriteId::x61_BEAMOS => vec![[
            SpritesheetId::None,
            SpritesheetId::x2C_BEAM_ME_UP_MR_POPO,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x62_MASTERSWORD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x37_MASTER_SWORD,
            SpritesheetId::x36_MASTER_SWORD,
        ]],
        SpriteId::x63_DEVALANT_PIT => vec![[
            SpritesheetId::x2F_CANON_SANDCRAB,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x64_DEVALANT => vec![[
            SpritesheetId::x2F_CANON_SANDCRAB,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x65_ARCHERY_GUY => vec![[
            SpritesheetId::x4B_ARCHERY,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x66_MOVING_CANNON_EAST => vec![[
            SpritesheetId::x2F_CANON_SANDCRAB,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x67_MOVING_CANNON_WEST => vec![[
            SpritesheetId::x2F_CANON_SANDCRAB,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x68_MOVING_CANNON_SOUTH => vec![[
            SpritesheetId::x2F_CANON_SANDCRAB,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x69_MOVING_CANNON_NORTH => vec![[
            SpritesheetId::x2F_CANON_SANDCRAB,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x6A_BALL_N_CHAIN_GUARD => vec![[
            SpritesheetId::x46_SOLDIERS,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x6B_CANNON_GUARD => vec![
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x46_SOLDIERS,
                SpritesheetId::xD_SOLDIERS_DW,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        // SpriteId::x6C_MIRROR_PORTAL
        SpriteId::x6D_RAT_CRICKET => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x1C_PESTS,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x24_PESTS_DW,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x6E_ROPE => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x1C_PESTS,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x24_PESTS_DW,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x6F_KEESE => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x1C_PESTS,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x24_PESTS_DW,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x70_HELMASAUR_KING_FIREBALL => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3A_HELMASAUR_KING_BOSS,
            SpritesheetId::x3E_HELMASAUR_KING_BOSS,
        ]],
        SpriteId::x71_LEEVER => vec![[
            SpritesheetId::x2F_CANON_SANDCRAB,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x72_FAIRY_POND_TRIGGER => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x36_MASTER_SWORD,
        ]],
        SpriteId::x73_UNCLE_PRIEST_MANTLE => vec![
            [
                SpritesheetId::x47_PRIEST,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x51_UNCLE_PRIEST_SICK_BOY,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x74_RUNNING_MAN => vec![[
            SpritesheetId::x4F_OLD_MAN_RUNNER,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x75_BOTTLE_SALESMAN => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4A_KAKARIKO,
            SpritesheetId::None,
        ]],
        // SpriteId::x76_ZELDA
        SpriteId::x77_ANTIFAIRY_2 => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x53_ObjectsHazards,
        ]],
        SpriteId::x78_VILLAGE_ELDER => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4A_KAKARIKO,
            SpritesheetId::None,
        ]],
        // SpriteId::x79_BEE
        SpriteId::x7A_AGAHNIM => vec![[
            SpritesheetId::x55_AgahnimCrystalMaiden,
            SpritesheetId::x1A_AGAHNIM,
            SpritesheetId::x42_AGAHNIM,
            SpritesheetId::x43_AGAHNIM,
        ]],
        SpriteId::x7B_AGAHNIM_ENERGY_BALL => vec![[
            SpritesheetId::x55_AgahnimCrystalMaiden,
            SpritesheetId::x1A_AGAHNIM,
            SpritesheetId::x42_AGAHNIM,
            SpritesheetId::x43_AGAHNIM,
        ]],
        SpriteId::x7C_FloatingStalfosHead => vec![
            // Homebrew: Soldier head.
            [
                SpritesheetId::x49_SOLDIERS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x1F_STALFOS_BARI,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x7D_BIG_SPIKE => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x52_ObjectsHazards,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x53_ObjectsHazards,
            ],
        ],
        SpriteId::x7E_FIREBAR_CLOCKWISE => vec![
            [
                SpritesheetId::x1F_STALFOS_BARI,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            // Homebrew: chain chomp bar.
            [
                SpritesheetId::x27_TURTLE_ROCK,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            // Homebrew: eyeball bar
            [
                SpritesheetId::x3C_KHOLDSTARE_BOSS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            // Homebrew: moldorm bar
            [
                SpritesheetId::x30_MOLDORM_BOSS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x7F_FIREBAR_COUNTER_CLOCKWISE => vec![
            [
                SpritesheetId::x1F_STALFOS_BARI,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            // Homebrew: chain chomp bar.
            [
                SpritesheetId::x27_TURTLE_ROCK,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            // Homebrew: eyeball bar
            [
                SpritesheetId::x3C_KHOLDSTARE_BOSS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            // Homebrew: moldorm bar
            [
                SpritesheetId::x30_MOLDORM_BOSS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x80_FIRESNAKE => vec![
            [
                SpritesheetId::x1F_STALFOS_BARI,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            // Homebrew: chain chomp snake.
            [
                SpritesheetId::x27_TURTLE_ROCK,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            // Homebrew: eyeball snake
            [
                SpritesheetId::x3C_KHOLDSTARE_BOSS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            // Homebrew: moldorm snake
            [
                SpritesheetId::x30_MOLDORM_BOSS,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x81_WATER_TEKTITE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x22_WATER_TEKTITES,
            SpritesheetId::None,
        ]],
        SpriteId::x82_ANTIFAIRY_CIRCLE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x53_ObjectsHazards,
        ]],
        SpriteId::x83_GREEN_EYEGORE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x2E_EyeGoreCannonBall,
            SpritesheetId::None,
        ]],
        SpriteId::x84_RED_EYEGORE => vec![[
            SpritesheetId::None,
            SpritesheetId::x2C_BEAM_ME_UP_MR_POPO,
            SpritesheetId::x2E_EyeGoreCannonBall,
            SpritesheetId::None,
        ]],
        SpriteId::x85_YELLOW_STALFOS => vec![[
            SpritesheetId::x1F_STALFOS_BARI,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x86_KODONGO => vec![[
            SpritesheetId::None,
            SpritesheetId::x2A_HAZARDS,
            SpritesheetId::x2A_HAZARDS,
            SpritesheetId::None,
        ]],
        SpriteId::x87_KODONGO_FIRE => vec![[
            SpritesheetId::None,
            SpritesheetId::x2A_HAZARDS,
            SpritesheetId::x2A_HAZARDS,
            SpritesheetId::None,
        ]],
        SpriteId::x88_MOTHULA => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x38_MOTHULA_BOSS,
            SpritesheetId::None,
        ]],
        SpriteId::x89_MOTHULA_BEAM => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x38_MOTHULA_BOSS,
            SpritesheetId::None,
        ]],
        SpriteId::x8A_SPIKE_BLOCK => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x52_ObjectsHazards,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x53_ObjectsHazards,
            ],
        ],
        SpriteId::x8B_GIBDO => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x23_WALLMASTER_GIBDO,
            SpritesheetId::None,
        ]],
        SpriteId::x8C_ARRGHUS => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x39_ARRGHUS_BOSS,
            SpritesheetId::None,
        ]],
        SpriteId::x8D_ARRGHUS_SPAWN => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x39_ARRGHUS_BOSS,
            SpritesheetId::None,
        ]],
        SpriteId::x8E_TERRORPIN => vec![[
            SpritesheetId::None,
            SpritesheetId::x2A_HAZARDS,
            SpritesheetId::x2A_HAZARDS,
            SpritesheetId::None,
        ]],
        SpriteId::x8F_BLOB => vec![[
            SpritesheetId::None,
            SpritesheetId::x20_STALFOS_KNIGHT_VERMIN,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x90_WALLMASTER => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x23_WALLMASTER_GIBDO,
            SpritesheetId::None,
        ]],
        SpriteId::x91_STALFOS_KNIGHT => vec![[
            SpritesheetId::None,
            SpritesheetId::x20_STALFOS_KNIGHT_VERMIN,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x92_KING_HELMASAUR => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3A_HELMASAUR_KING_BOSS,
            SpritesheetId::x3E_HELMASAUR_KING_BOSS,
        ]],
        SpriteId::x93_BUMPER => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x52_ObjectsHazards,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x53_ObjectsHazards,
            ],
        ],
        SpriteId::x94_PIROGUSU => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x22_WATER_TEKTITES,
            SpritesheetId::None,
        ]],
        SpriteId::x95_EYE_LASER_EAST => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x52_ObjectsHazards,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x53_ObjectsHazards,
            ],
        ],
        SpriteId::x96_EYE_LASER_WEST => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x52_ObjectsHazards,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x53_ObjectsHazards,
            ],
        ],
        SpriteId::x97_EYE_LASER_SOUTH => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x52_ObjectsHazards,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x53_ObjectsHazards,
            ],
        ],
        SpriteId::x98_EYE_LASER_NORTH => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x52_ObjectsHazards,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x53_ObjectsHazards,
            ],
        ],
        SpriteId::x99_PENGATOR => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x26_FROSTY_FRIENDS,
            SpritesheetId::None,
        ]],
        SpriteId::x9A_KYAMERON => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x22_WATER_TEKTITES,
            SpritesheetId::None,
        ]],
        SpriteId::x9B_WIZZROBE => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x25_WIZZROBE_SLUGGULA,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x29_WIZZROBE,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x9C_BABASU_EAST => vec![[
            SpritesheetId::None,
            SpritesheetId::x20_STALFOS_KNIGHT_VERMIN,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x9D_BABUSU_SOUTH => vec![[
            SpritesheetId::None,
            SpritesheetId::x20_STALFOS_KNIGHT_VERMIN,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x9E_HAUNTED_GROVE_OSTRICH => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
            SpritesheetId::None,
        ]],
        SpriteId::x9F_HAUNTED_GROVE_RABBIT => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
            SpritesheetId::None,
        ]],
        SpriteId::xA0_HAUNTED_GROVE_BIRD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4E_OSTR_DUCK_RABBIT_FLUTEBOY,
            SpritesheetId::None,
        ]],
        SpriteId::xA1_FREEZOR => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x26_FROSTY_FRIENDS,
            SpritesheetId::None,
        ]],
        SpriteId::xA2_KHOLDSTARE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3C_KHOLDSTARE_BOSS,
            SpritesheetId::None,
        ]],
        SpriteId::xA3_KHOLDSTARES_SHELL => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3C_KHOLDSTARE_BOSS,
            SpritesheetId::None,
        ]],
        SpriteId::xA4_FALLING_ICE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3C_KHOLDSTARE_BOSS,
            SpritesheetId::None,
        ]],
        SpriteId::xA5_BLUE_ZAZAK => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x28_ZAZAK,
            SpritesheetId::None,
        ]],
        SpriteId::xA6_RED_ZAZAK => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x28_ZAZAK,
            SpritesheetId::None,
        ]],
        SpriteId::xA7_STALFOS => vec![[
            SpritesheetId::x1F_STALFOS_BARI,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::xA8_GREEN_ZIRRO => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x1B_MISCELLANEOUS_DW_1,
        ]],
        SpriteId::xA9_BLUE_ZIRRO => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x1B_MISCELLANEOUS_DW_1,
        ]],
        SpriteId::xAA_PIKIT_LIKE_LIKE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x1B_MISCELLANEOUS_DW_1,
        ]],
        SpriteId::xAB_CRYSTAL_MAIDEN => vec![[
            SpritesheetId::None,
            SpritesheetId::x65_FollowerOldManMaiden,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        // SpriteId::xAC_APPLE
        SpriteId::xAD_LOST_OLD_MAN => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x1C_PESTS,
            SpritesheetId::None,
        ]],
        // SpriteId::xB0_RIGHT_PIPE
        // SpriteId::xB1_LEFT_PIPE
        // SpriteId::xB2_GOOD_BEE
        SpriteId::xB3_PEDESTAL_PLAQUE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x12_DESERT_1,
            SpritesheetId::None,
        ]],
        SpriteId::xB4_PURPLE_CHEST => vec![[
            SpritesheetId::x15_POE_THIEF_DW,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x15_POE_THIEF_DW,
        ]],
        SpriteId::xB5_BOMB_SALESMAN => vec![[
            SpritesheetId::None,
            SpritesheetId::x4D_OLD_MAN_MAIDEN,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::xB6_KIKI => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x19_SWAMOLA_CROW,
        ]],
        SpriteId::xB7_BLIND_MAIDEN => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::xB8_Goriya => vec![[
            SpritesheetId::None,
            SpritesheetId::x2C_BEAM_ME_UP_MR_POPO,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::xB9_BULLY_AND_FRIEND => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x14_FRIENDLY_LYNEL,
        ]],
        SpriteId::xBA_WHIRLPOOL => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::xC_OCTOROK_ZORA,
            SpritesheetId::None,
        ]],
        SpriteId::xBB_SALESMAN => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x4A_KAKARIKO,
            SpritesheetId::None,
        ]],
        SpriteId::xBC_DRUNK_IN_THE_INN => vec![[
            SpritesheetId::x4F_OLD_MAN_RUNNER,
            SpritesheetId::None,
            SpritesheetId::x4A_KAKARIKO,
            SpritesheetId::None,
        ]],
        SpriteId::xBD_VITREOUS => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3D_VITREOUS_BOSS,
        ]],
        SpriteId::xBE_VITREOUS_SMALL_EYEBALL => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3D_VITREOUS_BOSS,
        ]],
        SpriteId::xBF_LIGHTNING => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3D_VITREOUS_BOSS,
        ]],
        SpriteId::xC0_CATFISH => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x18_OCTOROCKS,
            SpritesheetId::None,
        ]],
        SpriteId::xC1_AGAHNIM_TELEPORTING => vec![[
            SpritesheetId::x55_AgahnimCrystalMaiden,
            SpritesheetId::x1A_AGAHNIM,
            SpritesheetId::x42_AGAHNIM,
            SpritesheetId::x43_AGAHNIM,
        ]],
        SpriteId::xC2_BOULDER => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::xC3_GIBO => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x28_ZAZAK,
            SpritesheetId::None,
        ]],
        SpriteId::xC4_THIEF => vec![
            [
                SpritesheetId::xE_POE_THIEF_LW,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::x15_POE_THIEF_DW,
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::None,
            ],
        ],
        // SpriteId::xC6_MEDUSA_FOUR_WAY
        SpriteId::xC7_POKEY => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x27_TURTLE_ROCK,
            SpritesheetId::None,
        ]],
        SpriteId::xC8_GREAT_FAIRY => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x39_ARRGHUS_BOSS,
            SpritesheetId::None,
        ]],
        SpriteId::xC9_TEKTITE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::xCA_CHAIN_CHOMP => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x27_TURTLE_ROCK,
            SpritesheetId::None,
        ]],
        SpriteId::xCB_TRINEXX_ROCK => vec![[
            SpritesheetId::x40_TRINEXX_BOSS,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3F_TRINEXX_BOSS,
        ]],
        SpriteId::xCC_TRINEXX_FIRE => vec![[
            SpritesheetId::x40_TRINEXX_BOSS,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3F_TRINEXX_BOSS,
        ]],
        SpriteId::xCD_TRINEXX_ICE => vec![[
            SpritesheetId::x40_TRINEXX_BOSS,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x3F_TRINEXX_BOSS,
        ]],
        SpriteId::xCE_BLIND => vec![[
            SpritesheetId::None,
            SpritesheetId::x2C_BEAM_ME_UP_MR_POPO, // Frickin laser beams.
            SpritesheetId::x3B_BLIND_BOSS,
            SpritesheetId::None,
        ]],
        SpriteId::xCF_SWAMOLA => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x19_SWAMOLA_CROW,
        ]],
        SpriteId::xD0_LYNEL => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x14_FRIENDLY_LYNEL,
        ]],
        // SpriteId::xD1_BUNNY_BEAM
        SpriteId::xD2_FLOPPING_FISH => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::xC_OCTOROK_ZORA,
            SpritesheetId::None,
        ]],
        // SpriteId::xD3_STAL
        SpriteId::xD4_LANDMINE => vec![],
        SpriteId::xD5_DIGGING_GAME_PROPRIETOR => vec![[
            SpritesheetId::None,
            SpritesheetId::x2A_HAZARDS,
            SpritesheetId::x2A_HAZARDS,
            SpritesheetId::None,
        ]],
        SpriteId::xD6_GANON => vec![[
            SpritesheetId::x21_BIG_BAD_GUY,
            SpritesheetId::x41_BIG_BAD_GUY,
            SpritesheetId::x45_BIG_BAD_GUY,
            SpritesheetId::x33_BIG_BAD_GUY,
        ]],
        SpriteId::xD7_GANON_INVINCIBLE => vec![[
            SpritesheetId::x21_BIG_BAD_GUY,
            SpritesheetId::x41_BIG_BAD_GUY,
            SpritesheetId::x45_BIG_BAD_GUY,
            SpritesheetId::x33_BIG_BAD_GUY,
        ]],
        // SpriteId::xD8_HEART
        // SpriteId::xD9_GREEN_RUPEE
        // SpriteId::xDA_BLUE_RUPEE
        // SpriteId::xDB_RED_RUPEE
        // SpriteId::xDC_BOMB_REFILL_1
        // SpriteId::xDD_BOMB_REFILL_4
        // SpriteId::xDE_BOMB_REFILL_8
        // &SpriteId::xDF_SMALL_MAGIC_REFILL
        // SpriteId::xE0_FULL_MAGIC_REFILL
        // SpriteId::xE1_ARROW_REFILL_5
        // SpriteId::xE2_ARROW_REFILL_10
        // SpriteId::xE3_FAIRY
        // SpriteId::xE4_SMALL_KEY
        // SpriteId::xE5_BIG_KEY
        // SpriteId::xE6_SHIELD
        SpriteId::xE7_MUSHROOM => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x11_MISC_FAKE_SWORD,
        ]],
        SpriteId::xE8_FAKE_MASTER_SWORD => vec![[
            SpritesheetId::None,
            SpritesheetId::x49_SOLDIERS,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::xE9_MAGIC_MERCHANT => vec![[
            SpritesheetId::x4B_ARCHERY,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        // SpriteId::xEA_HEART_CONTAINER
        // SpriteId::xEB_HEART_PIECE
        // SpriteId::xEC_THROWN_ITEM
        SpriteId::xED_SOMARIA_PLATFORM => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x27_TURTLE_ROCK,
            SpritesheetId::None,
        ]],
        SpriteId::xEE_CASTLE_MANTLE => vec![[
            SpritesheetId::x5D_ObjectsFollowerThief,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::xEF_SOMARIA_PLATFORM_UNUSED_1 => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x27_TURTLE_ROCK,
            SpritesheetId::None,
        ]],
        SpriteId::xF0_SOMARIA_PLATFORM_UNUSED_2 => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x27_TURTLE_ROCK,
            SpritesheetId::None,
        ]],
        SpriteId::xF1_SOMARIA_PLATFORM_UNUSED_3 => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x27_TURTLE_ROCK,
            SpritesheetId::None,
        ]],
        SpriteId::xF2_MEDALLION_TABLET => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x12_DESERT_1,
            SpritesheetId::None,
        ]],
        SpriteId::xF3_PERSONS_DOOR_OW_OVERLORD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::xF4_FALLING_ROCKS_OW_OVERLORD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x10_MISC_ROCKS,
        ]],
        SpriteId::xF5_CANON_BALLS_OW_OVERLORD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x2E_EyeGoreCannonBall,
            SpritesheetId::None,
        ]],
        // SpriteId::xF6_UNKNOWN_F6_OW_OVERLORD
        // SpriteId::xF7_UNKNOWN_F7_OW_OVERLORD
        // SpriteId::xF8_UNKNOWN_F8_OW_OVERLORD
        // SpriteId::xF9_UNKNOWN_F9_OW_OVERLORD
        SpriteId::xFA_BLOB_DROP_OW_OVERLORD => vec![[
            SpritesheetId::None,
            SpritesheetId::x20_STALFOS_KNIGHT_VERMIN,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::xFB_OVERWORLD_WALLMASTER_OW_OVERLORD_DROPS_IN_HOULIHAN => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x23_WALLMASTER_GIBDO,
            SpritesheetId::None,
        ]],
        SpriteId::xFC_FLOOR_DROP_SQUARE_OW_OVERLORD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x52_ObjectsHazards,
        ]],
        SpriteId::xFD_FLOOR_DROP_NORTH_PATH_OW_OVERLORD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x52_ObjectsHazards,
        ]],
        SpriteId::xFE_FLOOR_DROP_EAST_PATH_OW_OVERLORD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x52_ObjectsHazards,
        ]],
        SpriteId::x102_CANON_BALLS_EP_4_WALL_CANONBALLS => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x2E_EyeGoreCannonBall,
            SpritesheetId::None,
        ]],
        SpriteId::x103_CANON_BALLS_EP_ENTRY => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x2E_EyeGoreCannonBall,
            SpritesheetId::None,
        ]],
        SpriteId::x104_ROPE_DROP_TRAP => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x1C_PESTS,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x24_PESTS_DW,
                SpritesheetId::None,
            ],
        ],
        SpriteId::x105_STALFOS_HEAD_TRAP => vec![[
            SpritesheetId::x1F_STALFOS_BARI,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        // SpriteId::x106_BOMB_DROP1_TRAP
        // SpriteId::x107_MOVING_FLOOR
        SpriteId::x108_TRANSFORMER_BUNNY_BEAM => vec![[
            SpritesheetId::None,
            SpritesheetId::x20_STALFOS_KNIGHT_VERMIN,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        SpriteId::x109_WALLMASTER_OVERLORD => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x23_WALLMASTER_GIBDO,
            SpritesheetId::None,
        ]],
        SpriteId::x10A_FLOOR_DROP_SQUARE => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x52_ObjectsHazards,
        ]],
        SpriteId::x10B_FLOOR_DROP_NORTH => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x52_ObjectsHazards,
        ]],
        SpriteId::x110_PIROGUSU_SPAWNER_RIGHT => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x22_WATER_TEKTITES,
            SpritesheetId::None,
        ]],
        SpriteId::x111_PIROGUSU_SPAWNER_LEFT => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x22_WATER_TEKTITES,
            SpritesheetId::None,
        ]],
        SpriteId::x112_PIROGUSU_SPAWNER_DOWN => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x22_WATER_TEKTITES,
            SpritesheetId::None,
        ]],
        SpriteId::x113_PIROGUSU_SPAWNER_UP => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x22_WATER_TEKTITES,
            SpritesheetId::None,
        ]],
        SpriteId::x114_FLYING_FLOOR_TILE_TRAP => vec![[
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::x52_ObjectsHazards,
        ]],
        SpriteId::x115_WIZZROBE_SPAWNER => vec![
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x25_WIZZROBE_SLUGGULA,
                SpritesheetId::None,
            ],
            [
                SpritesheetId::None,
                SpritesheetId::None,
                SpritesheetId::x29_WIZZROBE,
                SpritesheetId::None,
            ],
        ],
        // SpriteId::x116_ZORO_SPAWNER TODO
        // Spriteid::x117_FOUR_SKULL_TRAP_IN_POD_UNDER_POT
        SpriteId::x118_STALFOS_APPEAR => vec![[
            SpritesheetId::x1F_STALFOS_BARI,
            SpritesheetId::None,
            SpritesheetId::None,
            SpritesheetId::None,
        ]],
        _ => vec![],
    }
}
