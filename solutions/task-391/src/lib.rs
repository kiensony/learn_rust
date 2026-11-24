// rust-track: handwritten task-391
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 391;

pub fn closing_marks_removed_equals(text: &String) -> usize {
    closing_marks_removed_equals_impl(text)
}

fn closing_marks_removed_equals_impl(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_end_matches('=')
    }
    .len()
}
