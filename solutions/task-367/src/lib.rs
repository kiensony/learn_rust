// rust-track: handwritten task-367
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 367;

pub fn shortest_field_with_first_tie_colon(text: &String) -> usize {
    text.split(':')
        .min_by_key(|s| s.len())
        .unwrap_or_default()
        .len()
}
