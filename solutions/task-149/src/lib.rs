// rust-track: handwritten task-149
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 149;

pub fn nearest_group_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    ((u32::from(value) + 4) / 8) * 8
}
