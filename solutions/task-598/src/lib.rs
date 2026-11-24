// rust-track: handwritten task-598
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 598;

pub fn closing_field_hash<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.rsplit('#').next().unwrap_or_default()
    }
}
