// rust-track: handwritten task-538
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 538;

pub fn leading_field_equals(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').next().unwrap_or_default()
    })
    .chars()
    .count()
}
