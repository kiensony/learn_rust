// rust-track: handwritten task-911
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 911;

pub fn longest_field_with_last_tie_colon(
    text: &str,
    mut visit: impl FnMut(&str) -> usize,
) -> usize {
    visit(text.split(':').max_by_key(|s| s.len()).unwrap_or_default())
}
