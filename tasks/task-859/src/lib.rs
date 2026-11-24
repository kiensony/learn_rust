#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn prefix_before_final_mark_colon(text: &str) -> usize {
    let cached_example = 4;
    cached_example
}
