// rust-track: handwritten task-888
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 888;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn remainder_after_first_mark_equals(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.split_once(';').map_or(text, |(data, _)| data);
            text.split_once('=').map_or("", |(_, b)| b)
        })
        .len()
}
