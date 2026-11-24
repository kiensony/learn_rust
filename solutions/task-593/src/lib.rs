// rust-track: handwritten task-593
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 593;

pub fn prefix_before_final_mark_equals<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit_once('=').map_or("", |(a, _)| a)
    }
}
