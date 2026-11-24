// rust-track: handwritten task-379
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 379;

pub fn closing_field_pipe(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.rsplit('|').next().unwrap_or_default()
    }
    .len()
}
