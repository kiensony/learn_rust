// rust-track: handwritten task-297
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 297;

pub fn count_separators_plus(text: &str) -> String {
    text.matches('+').count().to_string()
}
