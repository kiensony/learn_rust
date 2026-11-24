// rust-track: handwritten task-878
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 878;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn remainder_after_first_mark_pipe(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.lines().last().unwrap_or_default();
            text.split_once('|').map_or("", |(_, b)| b)
        })
        .len()
}
