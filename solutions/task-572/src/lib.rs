// rust-track: handwritten task-572
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 572;

pub fn remainder_after_first_mark_slash<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split_once('/').map_or("", |(_, b)| b)
    }
}
