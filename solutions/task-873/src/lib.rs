// rust-track: handwritten task-873
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 873;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn leading_field_pipe(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.split('|').next().unwrap_or_default()
        })
        .len()
}
