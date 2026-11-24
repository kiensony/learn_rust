// rust-track: handwritten task-114
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 114;

pub fn round_up_2(value: u16) -> u32 {
    u32::from(value).div_ceil(2) * 2
}
