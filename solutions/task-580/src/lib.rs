// rust-track: handwritten task-580
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 580;

pub fn closing_marks_removed_pipe<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_end_matches('|')
    }
}
