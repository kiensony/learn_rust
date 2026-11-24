// rust-track: handwritten task-118
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 118;

pub fn digit_place_2(value: u16) -> u32 {
    (u32::from(value) / 2) % 2
}
