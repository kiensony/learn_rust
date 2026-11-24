// rust-track: handwritten task-277
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 277;

pub fn count_separators_star(text: &str) -> String {
    text.matches('*').count().to_string()
}
