// rust-track: handwritten task-371
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 371;

pub fn closing_marks_removed_slash(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_end_matches('/')
    }
    .len()
}
