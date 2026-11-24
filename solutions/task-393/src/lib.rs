// rust-track: handwritten task-393
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 393;

pub fn remainder_after_first_mark_equals(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split_once('=').map_or("", |(_, b)| b)
    }
    .len()
}
