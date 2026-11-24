// rust-track: handwritten task-129
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 129;

pub fn nearest_group_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    ((u32::from(value) + 1) / 3) * 3
}
