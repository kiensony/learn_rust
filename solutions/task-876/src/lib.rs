// rust-track: handwritten task-876
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 876;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn closing_marks_removed_pipe(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.trim_end_matches('|')
        })
        .len()
}
