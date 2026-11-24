// rust-track: handwritten task-128
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 128;

pub fn digit_place_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    (u32::from(value) / 3) % 3
}
