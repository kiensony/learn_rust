// rust-track: handwritten task-912
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 912;

pub fn shortest_field_with_first_tie_colon(
    text: &str,
    mut visit: impl FnMut(&str) -> usize,
) -> usize {
    visit(text.split(':').min_by_key(|s| s.len()).unwrap_or_default())
}
