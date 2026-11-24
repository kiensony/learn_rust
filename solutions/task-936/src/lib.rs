// rust-track: handwritten task-936
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 936;

pub fn closing_marks_removed_equals(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    closing_marks_removed_equals_impl(text, visit)
}

fn closing_marks_removed_equals_impl(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_end_matches('=')
    })
}
