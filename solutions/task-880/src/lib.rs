// rust-track: handwritten task-880
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 880;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn first_nonempty_field_pipe(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.split('|').find(|s| !s.is_empty()).unwrap_or_default()
        })
        .len()
}
