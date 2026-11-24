// rust-track: handwritten task-577
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 577;

pub fn leading_field_pipe<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').next().unwrap_or_default()
    }
}
