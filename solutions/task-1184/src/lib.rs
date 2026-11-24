// rust-track: handwritten task-1184
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1184;

pub fn closing_marks_removed_equals<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    closing_marks_removed_equals_impl(text, fallback)
}

fn closing_marks_removed_equals_impl<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_end_matches('=')
    };
    if found.is_empty() { fallback } else { found }
}
