// rust-track: handwritten task-571
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 571;

pub fn marks_removed_at_both_ends_slash<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_matches('/')
    }
}
