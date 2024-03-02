//! This file provides a list of normal entrances in the vanilla game.
//! NOTE: do not depend on the entrance ids being tied to particular dungeons or overworld
//! locations. Many Zelda mods randomize locations.

use strum_macros::Display;
use strum_macros::EnumIter;

use super::dungeon::DungeonId;
use super::dungeon_blockset_id::UWBlocksetId;
use super::uw_room_id::UWRoomId;

#[derive(Debug, Clone)]
pub(crate) struct Entrance {
    pub id: EntranceId,
    /// The underworld room this overworld entrance leads to.
    pub room_id: UWRoomId,
    /// The dungeon that is associated with this entrance.
    pub dungeon_id: DungeonId,
    /// The default blockset of the dungeon. This must be set on all rooms when
    /// altering the blockset.
    pub blockset_id: UWBlocksetId,
    /// The song that is played after entering the entrance.
    pub song_id: u8,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Hash, EnumIter, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub(crate) enum EntranceId {
    X00_Links_House = 0x00,
    X01_Sanctuary = 0x01,
    X02_Hyrule_Castle_Entrance_West = 0x02,
    X03_Hyrule_Castle_Entrance_South = 0x03,
    X04_Hyrule_Castle_Entrance_East = 0x04,
    X05_Old_Man_Cave_West = 0x05,
    X06_Old_Man_Cave_East = 0x06,
    X07_Eastern_Palace = 0x07,
    X08_Desert_Palace_Entrance_South = 0x08,
    X09_Desert_Palace_Entrance_East = 0x09,
    X0A_Desert_Palace_Entrance_West = 0x0A,
    X0B_Desert_Palace_Entrance_North = 0x0B,
    X0C_Elder_House_West = 0x0C,
    X0D_Elder_House_East = 0x0D,
    X0E_Two_Brothers_House_West = 0x0E,
    X0F_Two_Brothers_House_East = 0x0F,
    X10_Bat_Cave_Cave = 0x10,
    X11_Lumberjack_Tree_Cave = 0x11,
    X12_Superbunny_Cave_Bottom = 0x12,
    X13_Superbunny_Cave_Top = 0x13,
    X14_Dark_Death_Mountain_Ledge_West = 0x14,
    X15_Bumper_Cave_Bottom = 0x15,
    X16_Bumper_Cave_Top = 0x16,
    X17_Turtle_Rock_Isolated_Ledge_Entrance = 0x17,
    X18_Dark_Death_Mountain_Ledge_East = 0x18,
    X19_Fairy_Ascension_Cave_Bottom = 0x19,
    X1A_Fairy_Ascension_Cave_Top = 0x1A,
    X1B_Spiral_Cave_Bottom = 0x1B,
    X1C_Spiral_Cave = 0x1C,
    X1D_Paradox_Cave_Bottom = 0x1D,
    X1E_Paradox_Cave_Middle = 0x1E,
    X1F_Paradox_Cave_Top = 0x1F,
    X20_Spectacle_Rock_Cave_Bottom = 0x20,
    X21_Spectacle_Rock_Cave = 0x21,
    X22_Spectacle_Rock_Cave_Peak = 0x22,
    X23_Agahnims_Tower = 0x23,
    X24_Swamp_Palace = 0x24,
    X25_Palace_of_Darkness = 0x25,
    X26_Misery_Mire = 0x26,
    X27_Skull_Woods_Second_Section_Door_West = 0x27,
    X28_Skull_Woods_Second_Section_Door_East = 0x28,
    X29_Skull_Woods_First_Section_Door = 0x29,
    X2A_Skull_Woods_Final_Section = 0x2A,
    X2B_Lost_Woods_Hideout_Stump = 0x2B,
    X2C_Ice_Palace = 0x2C,
    X2D_Death_Mountain_Return_Cave_West = 0x2D,
    X2E_Death_Mountain_Return_Cave_East = 0x2E,
    X2F_Old_Man_House_Bottom = 0x2F,
    X30_Old_Man_House_Top = 0x30,
    X31_Hyrule_Castle_Secret_Entrance_Stairs = 0x31,
    X32_Tower_of_Hera = 0x32,
    X33_Thieves_Town = 0x33,
    X34_Turtle_Rock = 0x34,
    X35_Pyramid_Entrance = 0x35,
    X36_Ganons_Tower = 0x36,
    X37_North_Fairy_Cave = 0x37,
    X38_Kakariko_Well_Cave = 0x38,
    X39_Hookshot_Cave = 0x39,
    X3A_Hookshot_Cave_Back_Entrance = 0x3A,
    X3B_Lost_Woods_Gamble = 0x3B,
    X3C_Hype_Cave = 0x3C,
    X3D_Snitch_Lady_East = 0x3D,
    X3E_Snitch_Lady_West = 0x3E,
    X3F_Sick_Kids_House = 0x3F,
    X40_Spike_Cave = 0x40,
    X41_Tavern_Front = 0x41,
    X42_Tavern_North = 0x42,
    X43_Bush_Covered_House = 0x43,
    X44_Sahasrahlas_Hut = 0x44,
    X45_Kakariko_Shop = 0x45,
    X46_Chest_Game = 0x46,
    X47_Brewery = 0x47,
    X48_Library = 0x48,
    X49_Light_World_Bomb_Hut = 0x49,
    X4A_Chicken_House = 0x4A,
    X4B_Potion_Shop = 0x4B,
    X4D_Dam = 0x4D,
    X4E_Mimic_Cave = 0x4E,
    X4F_Hookshot_Fairy = 0x4F,
    X50_Cave_45 = 0x50,
    X51_Graveyard_Cave = 0x51,
    X52_Big_Bomb_Shop = 0x52,
    X53_C_Shaped_House = 0x53,
    X54_Long_Fairy_Cave = 0x54,
    X55_Dark_Desert_Fairy = 0x55,
    X56_Dark_World_Lumberjack_Shop = 0x56,
    X57_Cave_Shop_Lake_Hylia = 0x57,
    X58_Archery_Game = 0x58,
    X59_Dark_Sanctuary_Hint = 0x59,
    X5A_Kings_Grave = 0x5A,
    X5B_Waterfall_of_Wishing = 0x5B,
    X5C_Capacity_Upgrade = 0x5C,
    X5D_Lake_Hylia_Fairy = 0x5D,
    X5E_Mire_Shed = 0x5E,
    X5F_Village_of_Outcasts_Shop = 0x5F,
    X60_Blinds_Hideout = 0x60,
    X61_Dark_Desert_Hint = 0x61,
    X62_Pyramid_Fairy = 0x62,
    X63_Blacksmiths_Hut = 0x63,
    X64_Fortune_Teller_Light = 0x64,
    X65_Fortune_Teller_Dark = 0x65,
    X66_Kakariko_Gamble_Game = 0x66,
    X67_Palace_of_Darkness_Hint = 0x67,
    X68_East_Dark_World_Hint = 0x68,
    X69_Dark_Lake_Hylia_Ledge_Hint = 0x69,
    X6A_Good_Bee_Cave = 0x6A,
    X6B_Light_Hype_Fairy = 0x6B,
    X6C_Dark_Lake_Hylia_Fairy = 0x6C,
    X6D_Cave_Shop_Dark_Death_Mountain = 0x6D,
    X6E_Dark_World_Potion_Shop = 0x6E,
    X6F_Dark_Death_Mountain_Fairy = 0x6F,
    X70_Aginahs_Cave = 0x70,
    X71_Desert_Fairy = 0x71,
    X72_Lake_Hylia_Fortune_Teller = 0x72,
    X73_Dark_Lake_Hylia_Shop = 0x73,
    X74_Red_Shield_Shop = 0x74,
    X75_Lumberjack_House = 0x75,
    X76_Bonk_Fairy_Light = 0x76,
    X77_Bonk_Fairy_Dark = 0x77,
    X78_50_Rupee_Cave = 0x78,
    X79_Bonk_Rock_Cave = 0x79,
    X7A_20_Rupee_Cave = 0x7A,
    X7B_Dark_Lake_Hylia_Ledge_Spike_Cave = 0x7B,
    X7C_Mini_Moldorm_Cave = 0x7C,
    X7D_Checkerboard_Cave = 0x7D,
    X7E_Dark_World_Hammer_Peg_Cave = 0x7E,
    X7F_Ice_Rod_Cave = 0x7F,
    X80_Dark_Lake_Hylia_Ledge_Fairy = 0x80,
}

#[allow(dead_code)]
/// Provide the typical string identifier in randomizers for a particular location.
pub(crate) fn get_display_name(entrance_id: EntranceId) -> &'static str {
    match entrance_id {
        EntranceId::X00_Links_House => "Links House",
        EntranceId::X01_Sanctuary => "Sanctuary",
        EntranceId::X02_Hyrule_Castle_Entrance_West => "Hyrule Castle Entrance (West)",
        EntranceId::X03_Hyrule_Castle_Entrance_South => "Hyrule Castle Entrance (South)",
        EntranceId::X04_Hyrule_Castle_Entrance_East => "Hyrule Castle Entrance (East)",
        EntranceId::X05_Old_Man_Cave_West => "Old Man Cave (West)",
        EntranceId::X06_Old_Man_Cave_East => "Old Man Cave (East)",
        EntranceId::X07_Eastern_Palace => "Eastern Palace",
        EntranceId::X08_Desert_Palace_Entrance_South => "Desert Palace Entrance (South)",
        EntranceId::X09_Desert_Palace_Entrance_East => "Desert Palace Entrance (East)",
        EntranceId::X0A_Desert_Palace_Entrance_West => "Desert Palace Entrance (West)",
        EntranceId::X0B_Desert_Palace_Entrance_North => "Desert Palace Entrance (North)",
        EntranceId::X0C_Elder_House_West => "Elder House (West)",
        EntranceId::X0D_Elder_House_East => "Elder House (East)",
        EntranceId::X0E_Two_Brothers_House_West => "Two Brothers House (West)",
        EntranceId::X0F_Two_Brothers_House_East => "Two Brothers House (East)",
        EntranceId::X10_Bat_Cave_Cave => "Bat Cave Cave",
        EntranceId::X11_Lumberjack_Tree_Cave => "Lumberjack Tree Cave",
        EntranceId::X12_Superbunny_Cave_Bottom => "Superbunny Cave (Bottom)",
        EntranceId::X13_Superbunny_Cave_Top => "Superbunny Cave (Top)",
        EntranceId::X14_Dark_Death_Mountain_Ledge_West => "Dark Death Mountain Ledge (West)",
        EntranceId::X15_Bumper_Cave_Bottom => "Bumper Cave (Bottom)",
        EntranceId::X16_Bumper_Cave_Top => "Bumper Cave (Top)",
        EntranceId::X17_Turtle_Rock_Isolated_Ledge_Entrance => {
            "Turtle Rock Isolated Ledge Entrance"
        }
        EntranceId::X18_Dark_Death_Mountain_Ledge_East => "Dark Death Mountain Ledge (East)",
        EntranceId::X19_Fairy_Ascension_Cave_Bottom => "Fairy Ascension Cave (Bottom)",
        EntranceId::X1A_Fairy_Ascension_Cave_Top => "Fairy Ascension Cave (Top)",
        EntranceId::X1B_Spiral_Cave_Bottom => "Spiral Cave (Bottom)",
        EntranceId::X1C_Spiral_Cave => "Spiral Cave",
        EntranceId::X1D_Paradox_Cave_Bottom => "Paradox Cave (Bottom)",
        EntranceId::X1E_Paradox_Cave_Middle => "Paradox Cave (Middle)",
        EntranceId::X1F_Paradox_Cave_Top => "Paradox Cave (Top)",
        EntranceId::X20_Spectacle_Rock_Cave_Bottom => "Spectacle Rock Cave (Bottom)",
        EntranceId::X21_Spectacle_Rock_Cave => "Spectacle Rock Cave",
        EntranceId::X22_Spectacle_Rock_Cave_Peak => "Spectacle Rock Cave Peak",
        EntranceId::X23_Agahnims_Tower => "Agahnims Tower",
        EntranceId::X24_Swamp_Palace => "Swamp Palace",
        EntranceId::X25_Palace_of_Darkness => "Palace of Darkness",
        EntranceId::X26_Misery_Mire => "Misery Mire",
        EntranceId::X27_Skull_Woods_Second_Section_Door_West => {
            "Skull Woods Second Section Door (West)"
        }
        EntranceId::X28_Skull_Woods_Second_Section_Door_East => {
            "Skull Woods Second Section Door (East)"
        }
        EntranceId::X29_Skull_Woods_First_Section_Door => "Skull Woods First Section Door",
        EntranceId::X2A_Skull_Woods_Final_Section => "Skull Woods Final Section",
        EntranceId::X2B_Lost_Woods_Hideout_Stump => "Lost Woods Hideout Stump",
        EntranceId::X2C_Ice_Palace => "Ice Palace",
        EntranceId::X2D_Death_Mountain_Return_Cave_West => "Death Mountain Return Cave (West)",
        EntranceId::X2E_Death_Mountain_Return_Cave_East => "Death Mountain Return Cave (East)",
        EntranceId::X2F_Old_Man_House_Bottom => "Old Man House (Bottom)",
        EntranceId::X30_Old_Man_House_Top => "Old Man House (Top)",
        EntranceId::X31_Hyrule_Castle_Secret_Entrance_Stairs => {
            "Hyrule Castle Secret Entrance Stairs"
        }
        EntranceId::X32_Tower_of_Hera => "Tower of Hera",
        EntranceId::X33_Thieves_Town => "Thieves Town",
        EntranceId::X34_Turtle_Rock => "Turtle Rock",
        EntranceId::X35_Pyramid_Entrance => "Pyramid Entrance",
        EntranceId::X36_Ganons_Tower => "Ganons Tower",
        EntranceId::X37_North_Fairy_Cave => "North Fairy Cave",
        EntranceId::X38_Kakariko_Well_Cave => "Kakariko Well Cave",
        EntranceId::X39_Hookshot_Cave => "Hookshot Cave",
        EntranceId::X3A_Hookshot_Cave_Back_Entrance => "Hookshot Cave Back Entrance",
        EntranceId::X3B_Lost_Woods_Gamble => "Lost Woods Gamble",
        EntranceId::X3C_Hype_Cave => "Hype Cave",
        EntranceId::X3D_Snitch_Lady_East => "Snitch Lady (East)",
        EntranceId::X3E_Snitch_Lady_West => "Snitch Lady (West)",
        EntranceId::X3F_Sick_Kids_House => "Sick Kids House",
        EntranceId::X40_Spike_Cave => "Spike Cave",
        EntranceId::X41_Tavern_Front => "Tavern (Front)",
        EntranceId::X42_Tavern_North => "Tavern North",
        EntranceId::X43_Bush_Covered_House => "Bush Covered House",
        EntranceId::X44_Sahasrahlas_Hut => "Sahasrahlas Hut",
        EntranceId::X45_Kakariko_Shop => "Kakariko Shop",
        EntranceId::X46_Chest_Game => "Chest Game",
        EntranceId::X47_Brewery => "Brewery",
        EntranceId::X48_Library => "Library",
        EntranceId::X49_Light_World_Bomb_Hut => "Light World Bomb Hut",
        EntranceId::X4A_Chicken_House => "Chicken House",
        EntranceId::X4B_Potion_Shop => "Potion Shop",
        EntranceId::X4D_Dam => "Dam",
        EntranceId::X4E_Mimic_Cave => "Mimic Cave",
        EntranceId::X4F_Hookshot_Fairy => "Hookshot Fairy",
        EntranceId::X50_Cave_45 => "Cave 45",
        EntranceId::X51_Graveyard_Cave => "Graveyard Cave",
        EntranceId::X52_Big_Bomb_Shop => "Big Bomb Shop",
        EntranceId::X53_C_Shaped_House => "C-Shaped House",
        EntranceId::X54_Long_Fairy_Cave => "Long Fairy Cave",
        EntranceId::X55_Dark_Desert_Fairy => "Dark Desert Fairy",
        EntranceId::X56_Dark_World_Lumberjack_Shop => "Dark World Lumberjack Shop",
        EntranceId::X57_Cave_Shop_Lake_Hylia => "Cave Shop (Lake Hylia)",
        EntranceId::X58_Archery_Game => "Archery Game",
        EntranceId::X59_Dark_Sanctuary_Hint => "Dark Sanctuary Hint",
        EntranceId::X5A_Kings_Grave => "Kings Grave",
        EntranceId::X5B_Waterfall_of_Wishing => "Waterfall of Wishing",
        EntranceId::X5C_Capacity_Upgrade => "Capacity Upgrade",
        EntranceId::X5D_Lake_Hylia_Fairy => "Lake Hylia Fairy",
        EntranceId::X5E_Mire_Shed => "Mire Shed",
        EntranceId::X5F_Village_of_Outcasts_Shop => "Village of Outcasts Shop",
        EntranceId::X60_Blinds_Hideout => "Blinds Hideout",
        EntranceId::X61_Dark_Desert_Hint => "Dark Desert Hint",
        EntranceId::X62_Pyramid_Fairy => "Pyramid Fairy",
        EntranceId::X63_Blacksmiths_Hut => "Blacksmiths Hut",
        EntranceId::X64_Fortune_Teller_Light => "Fortune Teller (Light)",
        EntranceId::X65_Fortune_Teller_Dark => "Fortune Teller (Dark)",
        EntranceId::X66_Kakariko_Gamble_Game => "Kakariko Gamble Game",
        EntranceId::X67_Palace_of_Darkness_Hint => "Palace of Darkness Hint",
        EntranceId::X68_East_Dark_World_Hint => "East Dark World Hint",
        EntranceId::X69_Dark_Lake_Hylia_Ledge_Hint => "Dark Lake Hylia Ledge Hint",
        EntranceId::X6A_Good_Bee_Cave => "Good Bee Cave",
        EntranceId::X6B_Light_Hype_Fairy => "Light Hype Fairy",
        EntranceId::X6C_Dark_Lake_Hylia_Fairy => "Dark Lake Hylia Fairy",
        EntranceId::X6D_Cave_Shop_Dark_Death_Mountain => "Cave Shop (Dark Death Mountain)",
        EntranceId::X6E_Dark_World_Potion_Shop => "Dark World Potion Shop",
        EntranceId::X6F_Dark_Death_Mountain_Fairy => "Dark Death Mountain Fairy",
        EntranceId::X70_Aginahs_Cave => "Aginahs Cave",
        EntranceId::X71_Desert_Fairy => "Desert Fairy",
        EntranceId::X72_Lake_Hylia_Fortune_Teller => "Lake Hylia Fortune Teller",
        EntranceId::X73_Dark_Lake_Hylia_Shop => "Dark Lake Hylia Shop",
        EntranceId::X74_Red_Shield_Shop => "Red Shield Shop",
        EntranceId::X75_Lumberjack_House => "Lumberjack House",
        EntranceId::X76_Bonk_Fairy_Light => "Bonk Fairy (Light)",
        EntranceId::X77_Bonk_Fairy_Dark => "Bonk Fairy (Dark)",
        EntranceId::X78_50_Rupee_Cave => "50 Rupee Cave",
        EntranceId::X79_Bonk_Rock_Cave => "Bonk Rock Cave",
        EntranceId::X7A_20_Rupee_Cave => "20 Rupee Cave",
        EntranceId::X7B_Dark_Lake_Hylia_Ledge_Spike_Cave => "Dark Lake Hylia Ledge Spike Cave",
        EntranceId::X7C_Mini_Moldorm_Cave => "Mini Moldorm Cave",
        EntranceId::X7D_Checkerboard_Cave => "Checkerboard Cave",
        EntranceId::X7E_Dark_World_Hammer_Peg_Cave => "Dark World Hammer Peg Cave",
        EntranceId::X7F_Ice_Rod_Cave => "Ice Rod Cave",
        EntranceId::X80_Dark_Lake_Hylia_Ledge_Fairy => "Dark Lake Hylia Ledge Fairy",
    }
}
