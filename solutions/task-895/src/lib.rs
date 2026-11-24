// rust-track: handwritten task-895
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 895;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn leading_marks_removed_hash(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text
                .split_once('[')
                .and_then(|(_, tail)| tail.split_once(']'))
                .map_or("", |(inside, _)| inside);
            text.trim_start_matches('#')
        })
        .len()
}
