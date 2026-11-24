// rust-track: handwritten task-565
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 565;

pub fn longest_field_with_last_tie_colon<'a>(text: &'a str) -> &'a str {
    text.split(':').max_by_key(|s| s.len()).unwrap_or_default()
}
