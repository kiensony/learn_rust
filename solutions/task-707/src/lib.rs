// rust-track: handwritten task-707
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 707;

pub fn reading(value: Option<i16>) -> i16 {
    match value {
        Some(n) => n,
        None => 0,
    }
}
