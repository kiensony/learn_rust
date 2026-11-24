// rust-track: handwritten task-885
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 885;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn leading_marks_removed_equals(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.split_once(';').map_or(text, |(data, _)| data);
            text.trim_start_matches('=')
        })
        .len()
}
