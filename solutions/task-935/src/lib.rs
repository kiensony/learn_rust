// rust-track: handwritten task-935
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 935;

pub fn leading_marks_removed_equals(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_start_matches('=')
    })
}
