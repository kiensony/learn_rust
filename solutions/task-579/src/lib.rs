// rust-track: handwritten task-579
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 579;

pub fn leading_marks_removed_pipe<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_start_matches('|')
    }
}
