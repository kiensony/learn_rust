// rust-track: handwritten task-951
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 951;

pub fn longest_field_with_last_tie_hash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split('#').max_by_key(|s| s.len()).unwrap_or_default()
    })
}
