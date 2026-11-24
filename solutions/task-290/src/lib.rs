// rust-track: handwritten task-290
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 290;

pub fn remove_a_prefix_plus(text: &str) -> String {
    text.strip_prefix("+").unwrap_or(text).to_owned()
}
