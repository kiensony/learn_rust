// rust-track: handwritten task-120
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 120;

pub fn fractional_fee_2(value: u16) -> u32 {
    u32::from(value) * 2 / 100
}
