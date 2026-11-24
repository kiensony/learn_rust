// rust-track: handwritten task-562
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 562;

pub fn marks_removed_at_both_ends_colon<'a>(text: &'a str) -> &'a str {
    text.trim_matches(':')
}
