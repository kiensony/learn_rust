// rust-track: handwritten task-978
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 978;

pub fn negative_at(values: &[i16]) -> Option<usize> {
    values.iter().position(|&n| n < 0)
}
