// rust-track: handwritten task-105
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 105;

pub fn mirror_a_coordinate_10(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    u32::from(value).abs_diff(10)
}
