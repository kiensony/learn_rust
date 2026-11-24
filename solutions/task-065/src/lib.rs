// rust-track: handwritten task-065
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 65;

pub fn mirror_a_coordinate_2(value: u16) -> u32 {
    u32::from(value).abs_diff(2)
}
