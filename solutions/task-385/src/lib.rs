// rust-track: handwritten task-385
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 385;

pub fn first_nonempty_field_pipe(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').find(|s| !s.is_empty()).unwrap_or_default()
    }
    .len()
}
