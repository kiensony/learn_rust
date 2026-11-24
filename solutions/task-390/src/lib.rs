// rust-track: handwritten task-390
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 390;

pub fn leading_marks_removed_equals(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_start_matches('=')
    }
    .len()
}
