// rust-track: handwritten task-561
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 561;

pub fn closing_marks_removed_colon<'a>(text: &'a str) -> &'a str {
    text.trim_end_matches(':')
}
