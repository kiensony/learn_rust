// rust-track: handwritten task-281
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 281;

pub fn remove_a_suffix_tilde(text: &str) -> String {
    text.strip_suffix("~").unwrap_or(text).to_owned()
}
