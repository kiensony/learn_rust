#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn negative_at(values: &[i16]) -> Option<usize> {
    values.iter().rposition(|&n| n < 0)
}
