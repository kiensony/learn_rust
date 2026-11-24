// rust-track: handwritten task-365
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 365;

pub fn first_nonempty_field_colon(text: &String) -> usize {
    text.split(':')
        .find(|s| !s.is_empty())
        .unwrap_or_default()
        .len()
}
