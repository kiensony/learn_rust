// rust-track: handwritten task-268
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 268;

pub fn normalize_spaces_hash(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join("#")
}
