// rust-track: handwritten task-875
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 875;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn leading_marks_removed_pipe(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.trim_start_matches('|')
        })
        .len()
}
