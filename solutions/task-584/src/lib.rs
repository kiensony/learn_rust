// rust-track: handwritten task-584
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 584;

pub fn first_nonempty_field_pipe<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').find(|s| !s.is_empty()).unwrap_or_default()
    }
}
