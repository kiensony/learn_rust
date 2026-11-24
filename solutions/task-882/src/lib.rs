// rust-track: handwritten task-882
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 882;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn shortest_field_with_first_tie_pipe(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.split('|').min_by_key(|s| s.len()).unwrap_or_default()
        })
        .len()
}
