// rust-track: handwritten task-535
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 535;

pub fn first_nonempty_field_pipe(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').find(|s| !s.is_empty()).unwrap_or_default()
    })
    .chars()
    .count()
}
