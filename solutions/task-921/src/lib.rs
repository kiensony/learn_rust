// rust-track: handwritten task-921
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 921;

pub fn longest_field_with_last_tie_slash(
    text: &str,
    mut visit: impl FnMut(&str) -> usize,
) -> usize {
    visit({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
    })
}
