use strum_macros::FromRepr;

#[repr(u16)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, FromRepr, Hash)]
pub(crate) enum UWObjectId {
    X000_Ceiling = 0x000,
    X001_WallTopNorth = 0x001,
    X002_WallTopSouth = 0x002,
    X003_WallBottomNorth = 0x003,
    X004_WallBottomSouth = 0x004,
    X005_WallColumnsNorth = 0x005,
    X006_WallColumnsSouth = 0x006,
    X007_DeepWallNorth = 0x007,
    X008_DeepWallSouth = 0x008,
    X009_DiagonalWallA = 0x009, // ◤_top
    X00A_DiagonalWallA = 0x00A, // ◣_top
    X00B_DiagonalWallA = 0x00B, // ◥_top
    X00C_DiagonalWallA = 0x00C, // ◢_top
    X00D_DiagonalWallB = 0x00D, // ◤_top
    X00E_DiagonalWallB = 0x00E, // ◣_top
    X00F_DiagonalWallB = 0x00F, // ◥_top
    X010_DiagonalWallB = 0x010, // ◢_top
    X011_DiagonalWallC = 0x011, // ◤_top
    X012_DiagonalWallC = 0x012, // ◣_top
    X013_DiagonalWallC = 0x013, // ◥_top
    X014_DiagonalWallC = 0x014, // ◢_top
    X015_DiagonalWallA = 0x015, // ◤_bottom
    X016_DiagonalWallA = 0x016, // ◣_bottom
    X017_DiagonalWallA = 0x017, // ◥_bottom
    X018_DiagonalWallA = 0x018, // ◢_bottom
    X019_DiagonalWallB = 0x019, // ◤_bottom
    X01A_DiagonalWallB = 0x01A, // ◣_bottom
    X01B_DiagonalWallB = 0x01B, // ◥_bottom
    X01C_DiagonalWallB = 0x01C, // ◢_bottom
    X01D_DiagonalWallC = 0x01D, // ◤_bottom
    X01E_DiagonalWallC = 0x01E, //  ◣_bottom
    X01F_DiagonalWallC = 0x01F, // ◥_bottom
    X020_DiagonalWallC = 0x020, // ◢_bottom
    X021_PlatformStairs = 0x021,
    X022_Rail = 0x022,
    X023_PitEdgeANorth = 0x023, // ┏━┓
    X024_PitEdgeBNorth = 0x024, // ┏━┓
    X025_PitEdgeCNorth = 0x025, // ┏━┓
    X026_PitEdgeDNorth = 0x026, // ┏━┓
    X027_PitEdgeENorth = 0x027, // ┏━┓
    X028_PitEdgeSouth = 0x028,  // ┗━┛
    X029_PitEdgeSouth = 0x029,  // ━━━
    X02A_PitEdgeNorth = 0x02A,  // ━━━
    X02B_PitEdgeSouth = 0x02B,  // ━━┛
    X02C_PitEdgeSouth = 0x02C,  // ┗━━
    X02D_PitEdgeNorth = 0x02D,  // ━━┓
    X02E_PitEdgeNorth = 0x02E,  // ┏━━
    X02F_RailWallNorth = 0x02F,
    X030_RailWallSouth = 0x030,
    X031_Nothing = 0x031,
    X032_Nothing = 0x032,
    X033_Carpet = 0x033,
    X034_CarpetTrim = 0x034,
    X035_WeirdDoor = 0x035,
    X036_DrapesNorth = 0x036,
    X037_DrapesWestOdd = 0x037,
    X038_Statues = 0x038,
    X039_Columns = 0x039,
    X03A_WallDecorsNorth = 0x03A,
    X03B_WallDecorsSouth = 0x03B,
    X03C_ChairsInPairs = 0x03C,
    X03D_TallTorches = 0x03D,
    X03E_SupportsNorth = 0x03E,
    X03F_WaterEdge = 0x03F, // ┏━┓_concave
    X040_WaterEdge = 0x040, // ┗━┛_concave
    X041_WaterEdge = 0x041, // ┏━┓_convex
    X042_WaterEdge = 0x042, // ┗━┛_convex
    X043_WaterEdge = 0x043, // ┏━┛_concave
    X044_WaterEdge = 0x044, // ┗━┓_concave
    X045_WaterEdge = 0x045, // ┗━┓_convex
    X046_WaterEdge = 0x046, // ┏━┛_convex
    X047_Unknown = 0x047,
    X048_Unknown = 0x048,
    X049_Unknown = 0x049,
    X04A_Unknown = 0x04A,
    X04B_SupportsSouth = 0x04B,
    X04C_Bar = 0x04C,
    X04D_ShelfA = 0x04D,
    X04E_ShelfB = 0x04E,
    X04F_ShelfC = 0x04F,
    X050_SomariaPath = 0x050,
    X051_CannonHoleANorth = 0x051,
    X052_CannonHoleASouth = 0x052,
    X053_PipePath = 0x053,
    X054_Nothing = 0x054,
    X055_WallTorchesNorth = 0x055,
    X056_WallTorchesSouth = 0x056,
    X057_Nothing = 0x057,
    X058_Nothing = 0x058,
    X059_Nothing = 0x059,
    X05A_Nothing = 0x05A,
    X05B_CannonHoleBNorth = 0x05B,
    X05C_CannonHoleBSouth = 0x05C,
    X05D_ThickRail = 0x05D,
    X05E_Blocks = 0x05E,
    X05F_LongRail = 0x05F,
    X060_Ceiling = 0x060,
    X061_WallTopWest = 0x061,
    X062_WallTopEast = 0x062,
    X063_WallBottomWest = 0x063,
    X064_WallBottomEast = 0x064,
    X065_WallColumnsWest = 0x065,
    X066_WallColumnsEast = 0x066,
    X067_DeepWallWest = 0x067,
    X068_DeepWallEast = 0x068,
    X069_Rail = 0x069,
    X06A_PitEdgeWest = 0x06A,
    X06B_PitEdgeEast = 0x06B,
    X06C_RailWallWest = 0x06C,
    X06D_RailWallEast = 0x06D,
    X06E_Nothing = 0x06E,
    X06F_Nothing = 0x06F,
    X070_Carpet = 0x070,
    X071_CarpetTrim = 0x071,
    X072_Nothing = 0x072,
    X073_DrapesWest = 0x073,
    X074_DrapesEast = 0x074,
    X075_Columns = 0x075,
    X076_WallDecorsWest = 0x076,
    X077_WallDecorsEast = 0x077,
    X078_SupportsWest = 0x078,
    X079_WaterEdgeWest = 0x079,
    X07A_WaterEdgeEast = 0x07A,
    X07B_SupportsEast = 0x07B,
    X07C_SomariaPath = 0x07C,
    X07D_PipePath = 0x07D,
    X07E_Nothing = 0x07E,
    X07F_WallTorchesWest = 0x07F,
    X080_WallTorchesEast = 0x080,
    X081_WallDecorsTightAWest = 0x081,
    X082_WallDecorsTightAEast = 0x082,
    X083_WallDecorsTightBWest = 0x083,
    X084_WallDecorsTightBEast = 0x084,
    X085_CannonHoleWest = 0x085,
    X086_CannonHoleEast = 0x086,
    X087_TallTorches = 0x087,
    X088_ThickRail = 0x088,
    X089_Blocks = 0x089,
    X08A_LongRail = 0x08A,
    X08B_JumpLedgeWest = 0x08B,
    X08C_JumpLedgeEast = 0x08C,
    X08D_RugTrimWest = 0x08D,
    X08E_RugTrimEast = 0x08E,
    X08F_Bar = 0x08F,
    X090_WallFlairWest = 0x090,
    X091_WallFlairEast = 0x091,
    X092_BluePegs = 0x092,
    X093_OrangePegs = 0x093,
    X094_InvisibleFloor = 0x094,
    X095_FakePots = 0x095,
    X096_HammerPegs = 0x096,
    X097_Nothing = 0x097,
    X098_Nothing = 0x098,
    X099_Nothing = 0x099,
    X09A_Nothing = 0x09A,
    X09B_Nothing = 0x09B,
    X09C_Nothing = 0x09C,
    X09D_Nothing = 0x09D,
    X09E_Nothing = 0x09E,
    X09F_Nothing = 0x09F,
    X0A0_DiagonalCeilingA = 0x0A0,    // ◤
    X0A1_DiagonalCeilingA = 0x0A1,    // ◣
    X0A2_DiagonalCeilingA = 0x0A2,    // ◥
    X0A3_DiagonalCeilingA = 0x0A3,    // ◢
    X0A4_Pit = 0x0A4,                 // ⇲
    X0A5_DiagonalLayer2MaskA = 0x0A5, // ◤
    X0A6_DiagonalLayer2MaskA = 0x0A6, // ◣
    X0A7_DiagonalLayer2MaskA = 0x0A7, // ◥
    X0A8_DiagonalLayer2MaskA = 0x0A8, // ◢
    X0A9_DiagonalLayer2MaskB = 0x0A9, // ◤
    X0AA_DiagonalLayer2MaskB = 0x0AA, // ◣
    X0AB_DiagonalLayer2MaskB = 0x0AB, // ◥
    X0AC_DiagonalLayer2MaskB = 0x0AC, // ◢
    X0AD_Nothing = 0x0AD,
    X0AE_Nothing = 0x0AE,
    X0AF_Nothing = 0x0AF,
    X0B0_JumpLedgeNorth = 0x0B0,
    X0B1_JumpLedgeSouth = 0x0B1,
    X0B2_Rug = 0x0B2,
    X0B3_RugTrimNorth = 0x0B3,
    X0B4_RugTrimSouth = 0x0B4,
    X0B5_ArcheryGameCurtains = 0x0B5,
    X0B6_WallFlairNorth = 0x0B6,
    X0B7_WallFlairSouth = 0x0B7,
    X0B8_BluePegs = 0x0B8,
    X0B9_OrangePegs = 0x0B9,
    X0BA_InvisibleFloor = 0x0BA,
    X0BB_FakePressurePlates = 0x0BB,
    X0BC_FakePots = 0x0BC,
    X0BD_HammerPegs = 0x0BD,
    X0BE_Nothing = 0x0BE,
    X0BF_Nothing = 0x0BF,
    X0C0_CeilingLarge = 0x0C0,        // ⇲
    X0C1_ChestPlatformTall = 0x0C1,   // ⇲
    X0C2_Layer2PitMaskLarge = 0x0C2,  // ⇲
    X0C3_Layer2PitMaskMedium = 0x0C3, // ⇲
    X0C4_Floor1 = 0x0C4,              // ⇲
    X0C5_Floor3 = 0x0C5,              // ⇲
    X0C6_Layer2MaskLarge = 0x0C6,     // ⇲
    X0C7_Floor4 = 0x0C7,              // ⇲
    X0C8_WaterFloor = 0x0C8,          // ⇲
    X0C9_FloodWaterMedium = 0x0C9,    // ⇲
    X0CA_ConveyorFloor = 0x0CA,       // ⇲
    X0CB_Nothing = 0x0CB,
    X0CC_Nothing = 0x0CC,
    X0CD_MovingWallWest = 0x0CD, // ⇲
    X0CE_MovingWallEast = 0x0CE, // ⇲
    X0CF_Nothing = 0x0CF,
    X0D0_Nothing = 0x0D0,
    X0D1_IcyFloorA = 0x0D1, // ⇲
    X0D2_IcyFloorB = 0x0D2, // ⇲
    X0D3_MovingWallFlag = 0x0D3,
    X0D4_MovingWallFlag = 0x0D4,
    X0D5_MovingWallFlag = 0x0D5,
    X0D6_MovingWallFlag = 0x0D6,
    X0D7_Layer2MaskMedium = 0x0D7,   // ⇲
    X0D8_FloodWaterLarge = 0x0D8,    // ⇲
    X0D9_Layer2SwimMask = 0x0D9,     // ⇲
    X0DA_FloodWaterBLarge = 0x0DA,   // ⇲
    X0DB_Floor2 = 0x0DB,             // ⇲
    X0DC_ChestPlatformShort = 0x0DC, // ⇲
    X0DD_TableRock = 0x0DD,          // ⇲
    X0DE_SpikeBlocks = 0x0DE,        // ⇲
    X0DF_SpikedFloor = 0x0DF,        // ⇲
    X0E0_Floor7 = 0x0E0,             // ⇲
    X0E1_TiledFloor = 0x0E1,         // ⇲
    X0E2_RupeeFloor = 0x0E2,         // ⇲
    X0E3_ConveyorUpwards = 0x0E3,    // ⇲
    X0E4_ConveyorDownwards = 0x0E4,  // ⇲
    X0E5_ConveyorLeftwards = 0x0E5,  // ⇲
    X0E6_ConveyorRightwards = 0x0E6, // ⇲
    X0E7_HeavyCurrentWater = 0x0E7,  // ⇲
    X0E8_Floor10 = 0x0E8,            // ⇲
    X0E9_Nothing = 0x0E9,
    X0EA_Nothing = 0x0EA,
    X0EB_Nothing = 0x0EB,
    X0EC_Nothing = 0x0EC,
    X0ED_Nothing = 0x0ED,
    X0EE_Nothing = 0x0EE,
    X0EF_Nothing = 0x0EF,
    X0F0_Nothing = 0x0F0,
    X0F1_Nothing = 0x0F1,
    X0F2_Nothing = 0x0F2,
    X0F3_Nothing = 0x0F3,
    X0F4_Nothing = 0x0F4,
    X0F5_Nothing = 0x0F5,
    X0F6_Nothing = 0x0F6,
    X0F7_Nothing = 0x0F7,
    X100_CornerTopConcave = 0x100,        // ▛
    X101_CornerTopConcave = 0x101,        // ▙
    X102_CornerTopConcave = 0x102,        // ▜
    X103_CornerTopConcave = 0x103,        // ▟
    X104_CornerTopConvex = 0x104,         // ▟
    X105_CornerTopConvex = 0x105,         // ▜
    X106_CornerTopConvex = 0x106,         // ▙
    X107_CornerTopConvex = 0x107,         // ▛
    X108_CornerBottomConcave = 0x108,     // ▛
    X109_CornerBottomConcave = 0x109,     // ▙
    X10A_CornerBottomConcave = 0x10A,     // ▜
    X10B_CornerBottomConcave = 0x10B,     // ▟
    X10C_CornerBottomConvex = 0x10C,      // ▟
    X10D_CornerBottomConvex = 0x10D,      // ▜
    X10E_CornerBottomConvex = 0x10E,      // ▙
    X10F_CornerBottomConvex = 0x10F,      // ▛
    X110_KinkedCornerNorthBottom = 0x110, // ▜
    X111_KinkedCornerSouthBottom = 0x111, // ▟
    X112_KinkedCornerNorthBottom = 0x112, // ▛
    X113_KinkedCornerSouthBottom = 0x113, // ▙
    X114_KinkedCornerWestBottom = 0x114,  // ▙
    X115_KinkedCornerWestBottom = 0x115,  // ▛
    X116_KinkedCornerEastBottom = 0x116,  // ▟
    X117_KinkedCornerEastBottom = 0x117,  // ▜
    X118_DeepCornerConcave = 0x118,       // ▛
    X119_DeepCornerConcave = 0x119,       // ▙
    X11A_DeepCornerConcave = 0x11A,       // ▜
    X11B_DeepCornerConcave = 0x11B,       // ▟
    X11C_LargeBrazier = 0x11C,
    X11D_Statue = 0x11D,
    X11E_StarTileDisabled = 0x11E,
    X11F_StarTileEnabled = 0x11F,
    X120_SmallTorchLit = 0x120,
    X121_Barrel = 0x121,
    X122_Unknown = 0x122,
    X123_Table = 0x123,
    X124_FairyStatue = 0x124,
    X125_Unknown = 0x125,
    X126_Unknown = 0x126,
    X127_Chair = 0x127,
    X128_Bed = 0x128,
    X129_Fireplace = 0x129,
    X12A_MarioPortrait = 0x12A,
    X12B_Unknown = 0x12B,
    X12C_Unknown = 0x12C,
    X12D_InterroomStairsUp = 0x12D,
    X12E_InterroomStairsDown = 0x12E,
    X12F_InterroomStairsBDown = 0x12F,
    X130_IntraroomStairsNorthB = 0x130,
    X131_IntraroomStairsNorthSeparateLayers = 0x131,
    X132_IntraroomStairsNorthMergedLayers = 0x132,
    X133_IntraroomStairsNorthSwimLayer = 0x133,
    X134_Block = 0x134,
    X135_WaterLadderNorth = 0x135,
    X136_WaterLadderSouth = 0x136,
    X137_DamFloodgate = 0x137,
    X138_InterroomSpiralStairsUpTop = 0x138,
    X139_InterroomSpiralStairsDownTop = 0x139,
    X13A_InterroomSpiralStairsUpBottom = 0x13A,
    X13B_InterroomSpiralStairsDownBottom = 0x13B,
    X13C_SanctuaryWallNorth = 0x13C,
    X13D_Unknown = 0x13D,
    X13E_Pew = 0x13E,
    X13F_MagicBatAltar = 0x13F,
    X200_WaterfallFaceEmpty = 0x200,
    X201_WaterfallFaceShort = 0x201,
    X202_WaterfallFaceLong = 0x202,
    X203_SomariaPathEndpoint = 0x203,
    X204_SomariaPathIntersection = 0x204, // ╋
    X205_SomariaPathCorner = 0x205,       // ┏
    X206_SomariaPathCorner = 0x206,       // ┗
    X207_SomariaPathCorner = 0x207,       // ┓
    X208_SomariaPathCorner = 0x208,       // ┛
    X209_SomariaPathIntersection = 0x209, // ┳
    X20A_SomariaPathIntersection = 0x20A, // ┻
    X20B_SomariaPathIntersection = 0x20B, // ┣
    X20C_SomariaPathIntersection = 0x20C, // ┫
    X20D_Unknown = 0x20D,
    X20E_SomariaPath2WayEndpoint = 0x20E,
    X20F_SomariaPathCrossover = 0x20F,
    X210_BabasuHoleNorth = 0x210,
    X211_BabasuHoleSouth = 0x211,
    X212_9BlueRupees = 0x212,
    X213_TelepathyTile = 0x213,
    X214_WarpDoor = 0x214,
    X215_KholdstaresShell = 0x215,
    X216_HammerPeg = 0x216,
    X217_PrisonCell = 0x217,
    X218_BigKeyLock = 0x218,
    X219_Chest = 0x219,
    X21A_ChestOpen = 0x21A,
    X21B_IntraroomStairsSouth = 0x21B,
    X21C_IntraroomStairsSouthSeparateLayers = 0x21C,
    X21D_IntraroomStairsSouthMergedLayers = 0x21D,
    X21E_InterroomStraightStairsUpNorthTop = 0x21E,
    X21F_InterroomStraightStairsDownNorthTop = 0x21F,
    X220_InterroomStraightStairsUpSouthTop = 0x220,
    X221_InterroomStraightStairsDownSouthTop = 0x221,
    X222_DeepCornerConvex = 0x222, // ▟
    X223_DeepCornerConvex = 0x223, // ▜
    X224_DeepCornerConvex = 0x224, // ▙
    X225_DeepCornerConvex = 0x225, // ▛
    X226_InterroomStraightStairsUpNorthBottom = 0x226,
    X227_InterroomStraightStairsDownNorthBottom = 0x227,
    X228_InterroomStraightStairsUpSouthBottom = 0x228,
    X229_InterroomStraightStairsDownSouthBottom = 0x229,
    X22A_LampCones = 0x22A,
    X22B_Unknown = 0x22B,
    X22C_LiftableLargeBlock = 0x22C,
    X22D_AgahnimsAltar = 0x22D,
    X22E_AgahnimsBossRoom = 0x22E,
    X22F_Pot = 0x22F,
    X230_Unknown = 0x230,
    X231_BigChest = 0x231,
    X232_BigChestOpen = 0x232,
    X233_IntraroomStairsSouthSwimLayer = 0x233,
    X234_Unknown = 0x234,
    X235_Unknown = 0x235,
    X236_Unknown = 0x236,
    X237_Unknown = 0x237,
    X238_Unknown = 0x238,
    X239_Unknown = 0x239,
    X23A_PipeEndSouth = 0x23A,
    X23B_PipeEndNorth = 0x23B,
    X23C_PipeEndEast = 0x23C,
    X23D_PipeEndWest = 0x23D,
    X23E_PipeCorner = 0x23E,           // ▛
    X23F_PipeCorner = 0x23F,           // ▙
    X240_PipeCorner = 0x240,           // ▜
    X241_PipeCorner = 0x241,           // ▟
    X242_PipeRockIntersection = 0x242, // ⯊
    X243_PipeRockIntersection = 0x243, // ⯋
    X244_PipeRockIntersection = 0x244, // ◖
    X245_PipeRockIntersection = 0x245, // ◗
    X246_PipeCrossover = 0x246,
    X247_BombableFloor = 0x247,
    X248_FakeBombableFloor = 0x248,
    X249_Unknown = 0x249,
    X24A_WarpTile = 0x24A,
    X24B_ToolRack = 0x24B,
    X24C_Furnace = 0x24C,
    X24D_TubWide = 0x24D,
    X24E_Anvil = 0x24E,
    X24F_WarpTileDisabled = 0x24F,
    X250_PressurePlate = 0x250,
    X251_Unknown = 0x251,
    X252_BluePeg = 0x252,
    X253_OrangePeg = 0x253,
    X254_FortuneTellerRoom = 0x254,
    X255_Unknown = 0x255,
    X256_BarCorner = 0x256, // ▛
    X257_BarCorner = 0x257, // ▙
    X258_BarCorner = 0x258, // ▜
    X259_BarCorner = 0x259, // ▟
    X25A_DecorativeBowl = 0x25A,
    X25B_TubTall = 0x25B,
    X25C_Bookcase = 0x25C,
    X25D_Range = 0x25D,
    X25E_Suitcase = 0x25E,
    X25F_BarBottles = 0x25F,
    X260_ArrowGameHoleWest = 0x260,
    X261_ArrowGameHoleEast = 0x261,
    X262_VitreousGooGraphics = 0x262,
    X263_FakePressurePlate = 0x263,
    X264_MedusaHead = 0x264,
    X265_4WayShooterBlock = 0x265,
    X266_Pit = 0x266,
    X267_WallCrackNorth = 0x267,
    X268_WallCrackSouth = 0x268,
    X269_WallCrackWest = 0x269,
    X26A_WallCrackEast = 0x26A,
    X26B_LargeDecor = 0x26B,
    X26C_WaterGrateNorth = 0x26C,
    X26D_WaterGrateSouth = 0x26D,
    X26E_WaterGrateWest = 0x26E,
    X26F_WaterGrateEast = 0x26F,
    X270_WindowSunlight = 0x270,
    X271_FloorSunlight = 0x271,
    X272_TrinexxsShell = 0x272,
    X273_Layer2MaskFull = 0x273,
    X274_BossEntrance = 0x274,
    X275_MinigameChest = 0x275,
    X276_GanonDoor = 0x276,
    X277_TriforceWallOrnament = 0x277,
    X278_TriforceFloorTiles = 0x278,
    X279_FreezorHole = 0x279,
    X27A_PileOfBones = 0x27A,
    X27B_VitreousGooDamage = 0x27B,
    X27C_ArrowTile = 0x27C, // ↑
    X27D_ArrowTile = 0x27D, // ↓
    X27E_ArrowTile = 0x27E, // →
    X27F_Nothing = 0x27F,
}