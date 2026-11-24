// rust-track: handwritten task-552
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 552;

pub fn marks_removed_at_both_ends_hash(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_matches('#')
    })
    .chars()
    .count()
}
