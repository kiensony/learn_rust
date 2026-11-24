// rust-track: handwritten task-907
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 907;

pub fn marks_removed_at_both_ends_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit(text.trim_matches(':'))
}
