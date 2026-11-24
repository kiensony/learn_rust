// rust-track: handwritten task-288
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 288;

pub fn normalize_spaces_tilde(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join("~")
}
