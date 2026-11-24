// rust-track: handwritten task-937
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 937;

pub fn marks_removed_at_both_ends_equals(
    text: &str,
    mut visit: impl FnMut(&str) -> usize,
) -> usize {
    visit({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_matches('=')
    })
}
