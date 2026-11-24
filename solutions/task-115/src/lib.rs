// rust-track: handwritten task-115
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 115;

pub fn top_up_2(value: u16) -> u32 {
    (2 - u32::from(value) % 2) % 2
}
