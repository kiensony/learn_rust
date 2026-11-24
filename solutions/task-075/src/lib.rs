// rust-track: handwritten task-075
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 75;

pub fn mirror_a_coordinate_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    u32::from(value).abs_diff(3)
}
