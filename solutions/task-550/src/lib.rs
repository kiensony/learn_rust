// rust-track: handwritten task-550
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 550;

pub fn leading_marks_removed_hash(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_start_matches('#')
    })
    .chars()
    .count()
}
