// rust-track: handwritten task-124
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 124;

pub fn round_up_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    u32::from(value).div_ceil(3) * 3
}
