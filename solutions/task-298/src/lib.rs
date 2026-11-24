// rust-track: handwritten task-298
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 298;

pub fn normalize_spaces_plus(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join("+")
}
