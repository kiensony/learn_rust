// rust-track: handwritten task-308
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 308;

pub fn normalize_spaces_bang(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join("!")
}
