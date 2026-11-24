// rust-track: handwritten task-405
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 405;

pub fn first_nonempty_field_hash(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split('#').find(|s| !s.is_empty()).unwrap_or_default()
    }
    .len()
}
