// rust-track: handwritten task-854
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 854;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn closing_field_colon(text: &str) -> usize {
    wire::HEADER_BYTES + (text.rsplit(':').next().unwrap_or_default()).len()
}
