// rust-track: handwritten task-373
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 373;

pub fn remainder_after_first_mark_slash(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split_once('/').map_or("", |(_, b)| b)
    }
    .len()
}
