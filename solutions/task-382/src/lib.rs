// rust-track: handwritten task-382
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 382;

pub fn marks_removed_at_both_ends_pipe(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_matches('|')
    }
    .len()
}
