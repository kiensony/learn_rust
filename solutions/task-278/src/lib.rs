// rust-track: handwritten task-278
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 278;

pub fn normalize_spaces_star(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join("*")
}
