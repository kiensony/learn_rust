#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn both_points(point: (i16, i16)) -> [(i16, i16); 2] {
    [point, (0, 0)]
}
