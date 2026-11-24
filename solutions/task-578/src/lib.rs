// rust-track: handwritten task-578
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 578;

pub fn closing_field_pipe<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.rsplit('|').next().unwrap_or_default()
    }
}
