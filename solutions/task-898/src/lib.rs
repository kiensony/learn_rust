// rust-track: handwritten task-898
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 898;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn remainder_after_first_mark_hash(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text
                .split_once('[')
                .and_then(|(_, tail)| tail.split_once(']'))
                .map_or("", |(inside, _)| inside);
            text.split_once('#').map_or("", |(_, b)| b)
        })
        .len()
}
