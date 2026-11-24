// rust-track: handwritten task-605
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 605;

pub fn longest_field_with_last_tie_hash<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split('#').max_by_key(|s| s.len()).unwrap_or_default()
    }
}
