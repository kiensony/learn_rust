// rust-track: handwritten task-570
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 570;

pub fn closing_marks_removed_slash<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_end_matches('/')
    }
}
