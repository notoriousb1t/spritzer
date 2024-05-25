use strum_macros::FromRepr;

#[derive(Clone)]
pub(crate) struct OWSecrets {
    pub(crate) light_world: Vec<Secret>,
    pub(crate) dark_world: Vec<Secret>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Secret {
    /// The x coordinate of the Secret. This must line up with layout objects.
    pub(crate) x: u8,
    /// The y coordinate of the Secret. This must line up with layout objects.
    pub(crate) y: u8,
    /// True if the item is on the lower layer. This is only true in dungeons.
    pub(crate) is_lower_layer: bool,
    /// The item or object under the pot.
    pub(crate) item: Option<HiddenItem>,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromRepr)]
pub(crate) enum HiddenItem {
    X01GreenRupee = 0x01,
    X02RockCrab = 0x02,
    X03Bee = 0x03,
    X04Random = 0x04,
    X05Bomb = 0x05,
    X06Heart = 0x06,
    X07BlueRupee = 0x07,
    X08SmallKey = 0x08,
    X09Arrows5 = 0x09,
    X0ABomb = 0x0A,
    X0BHeart = 0x0B,
    X0CSmallMagic = 0x0C,
    X0DFullMagic = 0x0D,
    X0ECucco = 0x0E,
    X0FGreenSoldier = 0x0F,
    X10Stal = 0x10,
    X11BlueSoldier = 0x11,
    X12Landmine = 0x12,
    X13Heart = 0x13,
    X14Fairy = 0x14,
    X15Heart = 0x15,
    X80Hole = 0x80,
    X82Warp = 0x82,
    X84Staircase = 0x84,
    X86Bombable = 0x86,
    X88Switch = 0x88,
}