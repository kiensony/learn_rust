// rust-track: handwritten task-868
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 868;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn remainder_after_first_mark_slash(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.strip_prefix("path/").unwrap_or(text);
            text.split_once('/').map_or("", |(_, b)| b)
        })
        .len()
}
