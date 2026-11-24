// rust-track: handwritten task-533
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 533;

pub fn remainder_after_first_mark_pipe(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split_once('|').map_or("", |(_, b)| b)
    })
    .chars()
    .count()
}
