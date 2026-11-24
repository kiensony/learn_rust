// rust-track: handwritten task-566
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 566;

pub fn shortest_field_with_first_tie_colon<'a>(text: &'a str) -> &'a str {
    text.split(':').min_by_key(|s| s.len()).unwrap_or_default()
}
