// rust-track: handwritten task-1153
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1153;

pub fn prefix(text: &str) -> &str {
    text.split(':').next().unwrap_or("")
}
