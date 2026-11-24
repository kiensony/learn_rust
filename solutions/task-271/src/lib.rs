// rust-track: handwritten task-271
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 271;

pub fn remove_a_suffix_star(text: &str) -> String {
    text.strip_suffix("*").unwrap_or(text).to_owned()
}
