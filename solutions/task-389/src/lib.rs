// rust-track: handwritten task-389
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 389;

pub fn closing_field_equals(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit('=').next().unwrap_or_default()
    }
    .len()
}
