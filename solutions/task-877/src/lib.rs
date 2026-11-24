// rust-track: handwritten task-877
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 877;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn marks_removed_at_both_ends_pipe(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.trim_matches('|')
        })
        .len()
}
