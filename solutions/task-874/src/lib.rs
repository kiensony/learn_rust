// rust-track: handwritten task-874
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 874;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn closing_field_pipe(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.rsplit('|').next().unwrap_or_default()
        })
        .len()
}
