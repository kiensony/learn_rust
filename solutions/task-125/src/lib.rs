// rust-track: handwritten task-125
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 125;

pub fn top_up_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    (3 - u32::from(value) % 3) % 3
}
