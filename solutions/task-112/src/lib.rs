// rust-track: handwritten task-112
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 112;

pub fn degrees(quarters: i16) -> f64 {
    f64::from(quarters) / 4.0
}
