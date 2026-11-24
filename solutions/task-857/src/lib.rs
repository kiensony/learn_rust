// rust-track: handwritten task-857
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 857;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn marks_removed_at_both_ends_colon(text: &str) -> usize {
    wire::HEADER_BYTES + (text.trim_matches(':')).len()
}
