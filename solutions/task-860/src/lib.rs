// rust-track: handwritten task-860
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 860;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn first_nonempty_field_colon(text: &str) -> usize {
    wire::HEADER_BYTES + (text.split(':').find(|s| !s.is_empty()).unwrap_or_default()).len()
}
