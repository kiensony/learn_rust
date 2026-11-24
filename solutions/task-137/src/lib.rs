// rust-track: handwritten task-137
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 137;

pub fn midpoint(first: i16, second: i16) -> f64 {
    (f64::from(first) + f64::from(second)) / 2.0
}
