#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn leading_field_hash(text: &str) -> usize {
    0
}
