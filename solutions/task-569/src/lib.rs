// rust-track: handwritten task-569
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 569;

pub fn leading_marks_removed_slash<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_start_matches('/')
    }
}
