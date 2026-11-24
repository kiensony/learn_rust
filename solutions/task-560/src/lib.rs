// rust-track: handwritten task-560
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 560;

pub fn title<'a>(primary: &'a str, fallback: &'a str) -> &'a str {
    if primary.is_empty() {
        fallback
    } else {
        primary
    }
}
