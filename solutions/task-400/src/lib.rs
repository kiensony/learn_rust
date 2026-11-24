// rust-track: handwritten task-400
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 400;

pub fn leading_marks_removed_hash(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_start_matches('#')
    }
    .len()
}
