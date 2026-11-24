// rust-track: handwritten task-881
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 881;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn longest_field_with_last_tie_pipe(text: &str) -> usize {
    longest_field_with_last_tie_pipe_impl(text)
}

fn longest_field_with_last_tie_pipe_impl(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
        })
        .len()
}
