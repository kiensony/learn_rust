// rust-track: handwritten task-923
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 923;

pub fn leading_field_pipe(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').next().unwrap_or_default()
    })
}
