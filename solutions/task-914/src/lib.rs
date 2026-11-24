// rust-track: handwritten task-914
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 914;

pub fn closing_field_slash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit('/').next().unwrap_or_default()
    })
}
