// rust-track: handwritten task-307
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 307;

pub fn count_separators_bang(text: &str) -> String {
    text.matches('!').count().to_string()
}
