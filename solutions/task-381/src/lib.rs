// rust-track: handwritten task-381
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 381;

pub fn closing_marks_removed_pipe(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_end_matches('|')
    }
    .len()
}
