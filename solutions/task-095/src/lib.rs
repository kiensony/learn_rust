// rust-track: handwritten task-095
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 95;

pub fn mirror_a_coordinate_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    u32::from(value).abs_diff(8)
}
