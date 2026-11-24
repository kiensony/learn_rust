// rust-track: handwritten task-396
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 396;

pub fn longest_field_with_last_tie_equals(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').max_by_key(|s| s.len()).unwrap_or_default()
    }
    .len()
}
