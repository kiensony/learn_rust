#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 113;

pub fn active(byte: u8) -> bool {
    byte as bool
}
