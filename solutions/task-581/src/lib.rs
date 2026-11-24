// rust-track: handwritten task-581
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 581;

pub fn marks_removed_at_both_ends_pipe<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_matches('|')
    }
}
