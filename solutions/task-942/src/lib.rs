// rust-track: handwritten task-942
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 942;

pub fn shortest_field_with_first_tie_equals(
    text: &str,
    mut visit: impl FnMut(&str) -> usize,
) -> usize {
    visit({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').min_by_key(|s| s.len()).unwrap_or_default()
    })
}
