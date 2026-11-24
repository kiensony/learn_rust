// rust-track: handwritten task-370
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 370;

pub fn leading_marks_removed_slash(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_start_matches('/')
    }
    .len()
}
