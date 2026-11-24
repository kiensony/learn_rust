// rust-track: handwritten task-861
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 861;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn longest_field_with_last_tie_colon(text: &str) -> usize {
    wire::HEADER_BYTES + (text.split(':').max_by_key(|s| s.len()).unwrap_or_default()).len()
}
