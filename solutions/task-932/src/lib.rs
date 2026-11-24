// rust-track: handwritten task-932
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 932;

pub fn shortest_field_with_first_tie_pipe(
    text: &str,
    mut visit: impl FnMut(&str) -> usize,
) -> usize {
    visit({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').min_by_key(|s| s.len()).unwrap_or_default()
    })
}
