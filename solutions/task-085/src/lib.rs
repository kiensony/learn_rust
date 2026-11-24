// rust-track: handwritten task-085
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 85;

pub fn mirror_a_coordinate_5(issued: u16, returned: u16) -> u32 {
    let value = issued.saturating_sub(returned);
    u32::from(value).abs_diff(5)
}
