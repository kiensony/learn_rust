// rust-track: handwritten task-906
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 906;

pub fn closing_marks_removed_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit(text.trim_end_matches(':'))
}
