// rust-track: handwritten task-512
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 512;

pub fn marks_removed_at_both_ends_colon(text: &str) -> usize {
    (text.trim_matches(':')).chars().count()
}
