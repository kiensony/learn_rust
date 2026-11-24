// rust-track: handwritten task-387
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 387;

pub fn shortest_field_with_first_tie_pipe(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').min_by_key(|s| s.len()).unwrap_or_default()
    }
    .len()
}
