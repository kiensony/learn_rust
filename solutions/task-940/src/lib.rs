// rust-track: handwritten task-940
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 940;

pub fn first_nonempty_field_equals(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').find(|s| !s.is_empty()).unwrap_or_default()
    })
}
