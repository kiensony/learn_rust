// rust-track: handwritten task-460
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 460;

pub fn height(point: (i16, i16)) -> i16 {
    point.1
}
