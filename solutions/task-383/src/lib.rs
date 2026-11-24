// rust-track: handwritten task-383
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 383;

pub fn remainder_after_first_mark_pipe(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split_once('|').map_or("", |(_, b)| b)
    }
    .len()
}
