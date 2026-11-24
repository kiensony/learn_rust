// rust-track: handwritten task-380
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 380;

pub fn leading_marks_removed_pipe(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_start_matches('|')
    }
    .len()
}
