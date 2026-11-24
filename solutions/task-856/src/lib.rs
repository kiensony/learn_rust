// rust-track: handwritten task-856
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 856;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn closing_marks_removed_colon(text: &str) -> usize {
    wire::HEADER_BYTES + (text.trim_end_matches(':')).len()
}
