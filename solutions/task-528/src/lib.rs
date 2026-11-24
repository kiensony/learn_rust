// rust-track: handwritten task-528
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 528;

pub fn leading_field_pipe(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').next().unwrap_or_default()
    })
    .chars()
    .count()
}
