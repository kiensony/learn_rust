// rust-track: handwritten task-946
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 946;

pub fn closing_marks_removed_hash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    closing_marks_removed_hash_impl(text, visit)
}

fn closing_marks_removed_hash_impl(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_end_matches('#')
    })
}
