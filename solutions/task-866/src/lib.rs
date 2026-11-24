// rust-track: handwritten task-866
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 866;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn closing_marks_removed_slash(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.strip_prefix("path/").unwrap_or(text);
            text.trim_end_matches('/')
        })
        .len()
}
