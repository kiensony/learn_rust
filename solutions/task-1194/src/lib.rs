// rust-track: handwritten task-1194
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1194;

pub fn closing_marks_removed_hash<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    closing_marks_removed_hash_impl(text, fallback)
}

fn closing_marks_removed_hash_impl<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_end_matches('#')
    };
    if found.is_empty() { fallback } else { found }
}
