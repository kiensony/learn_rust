// rust-track: handwritten task-853
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 853;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn leading_field_colon(text: &str) -> usize {
    wire::HEADER_BYTES + (text.split(':').next().unwrap_or_default()).len()
}
