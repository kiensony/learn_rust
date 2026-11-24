// rust-track: handwritten task-589
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 589;

pub fn leading_marks_removed_equals<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_start_matches('=')
    }
}
