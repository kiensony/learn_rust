// rust-track: handwritten task-551
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 551;

pub fn closing_marks_removed_hash(text: &str) -> usize {
    closing_marks_removed_hash_impl(text)
}

fn closing_marks_removed_hash_impl(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_end_matches('#')
    })
    .chars()
    .count()
}
