// rust-track: handwritten task-532
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 532;

pub fn marks_removed_at_both_ends_pipe(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_matches('|')
    })
    .chars()
    .count()
}
