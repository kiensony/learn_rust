// rust-track: handwritten task-587
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 587;

pub fn leading_field_equals<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').next().unwrap_or_default()
    }
}
