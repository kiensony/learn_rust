// rust-track: handwritten task-361
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 361;

pub fn closing_marks_removed_colon(text: &String) -> usize {
    text.trim_end_matches(':').len()
}
