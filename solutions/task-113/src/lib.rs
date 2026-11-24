// rust-track: handwritten task-113
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 113;

pub fn active(byte: u8) -> bool {
    byte != 0
}
