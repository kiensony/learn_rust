// rust-track: handwritten task-925
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 925;

pub fn leading_marks_removed_pipe(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_start_matches('|')
    })
}
