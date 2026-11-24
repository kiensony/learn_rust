// rust-track: handwritten task-362
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 362;

pub fn marks_removed_at_both_ends_colon(text: &String) -> usize {
    text.trim_matches(':').len()
}
