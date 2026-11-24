// rust-track: handwritten task-511
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 511;

pub fn closing_marks_removed_colon(text: &str) -> usize {
    (text.trim_end_matches(':')).chars().count()
}
