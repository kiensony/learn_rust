#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn midpoint(first: i16, second: i16) -> f64 {
    f64::from((i32::from(first) + i32::from(second)) / 2)
}
