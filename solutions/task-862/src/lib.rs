// rust-track: handwritten task-862
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 862;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn shortest_field_with_first_tie_colon(text: &str) -> usize {
    wire::HEADER_BYTES + (text.split(':').min_by_key(|s| s.len()).unwrap_or_default()).len()
}
