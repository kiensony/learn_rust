// rust-track: handwritten task-334
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 334;

pub fn both_points(point: (i16, i16)) -> [(i16, i16); 2] {
    [point; 2]
}
