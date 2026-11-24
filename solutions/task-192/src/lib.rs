// rust-track: handwritten task-192
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 192;

pub fn comfort_zone_8(start: u16, end: u16) -> bool {
    let value = start.abs_diff(end);
    (8..=10).contains(&value)
}
