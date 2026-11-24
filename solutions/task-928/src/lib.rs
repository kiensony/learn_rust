// rust-track: handwritten task-928
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 928;

pub fn remainder_after_first_mark_pipe(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split_once('|').map_or("", |(_, b)| b)
    })
}
