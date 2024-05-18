use strum_macros::Display;
use strum_macros::FromRepr;

#[repr(u8)]
#[derive(Clone, Copy, FromRepr, Display)]
#[allow(dead_code)]
pub enum RomMode {
    SlowLoRom = 0x20,
    SlowHiRom = 0x21,
    Sa1Rom = 0x23,
    FastLoRom = 0x30,
    FastHiRom = 0x31,
    Sdd1Rom = 0x32,
    ExHiRom = 0x35,
}
