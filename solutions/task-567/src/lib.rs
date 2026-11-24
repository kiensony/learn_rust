// rust-track: handwritten task-567
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 567;

pub fn left_caption<'a>(left: &'a str, _right: &str) -> &'a str {
    left.trim()
}
