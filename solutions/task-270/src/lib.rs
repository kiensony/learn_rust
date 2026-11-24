// rust-track: handwritten task-270
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 270;

pub fn remove_a_prefix_star(text: &str) -> String {
    text.strip_prefix("*").unwrap_or(text).to_owned()
}
