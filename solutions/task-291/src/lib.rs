// rust-track: handwritten task-291
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 291;

pub fn remove_a_suffix_plus(text: &str) -> String {
    text.strip_suffix("+").unwrap_or(text).to_owned()
}
