// rust-track: handwritten task-938
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 938;

pub fn remainder_after_first_mark_equals(
    text: &str,
    mut visit: impl FnMut(&str) -> usize,
) -> usize {
    visit({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split_once('=').map_or("", |(_, b)| b)
    })
}
