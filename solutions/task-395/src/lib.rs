// rust-track: handwritten task-395
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 395;

pub fn first_nonempty_field_equals(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').find(|s| !s.is_empty()).unwrap_or_default()
    }
    .len()
}
