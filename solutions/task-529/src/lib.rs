// rust-track: handwritten task-529
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 529;

pub fn closing_field_pipe(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.rsplit('|').next().unwrap_or_default()
    })
    .chars()
    .count()
}
