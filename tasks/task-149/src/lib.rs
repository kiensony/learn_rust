#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn nearest_group_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    ((u32::from(value.saturating_sub(1)) + 4) / 8) * 8
}
