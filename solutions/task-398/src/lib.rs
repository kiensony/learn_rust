// rust-track: handwritten task-398
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 398;

pub fn leading_field_hash(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split('#').next().unwrap_or_default()
    }
    .len()
}
