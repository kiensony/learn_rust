// rust-track: handwritten task-871
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 871;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn longest_field_with_last_tie_slash(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.strip_prefix("path/").unwrap_or(text);
            text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
        })
        .len()
}
