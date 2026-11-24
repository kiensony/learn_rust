// rust-track: handwritten task-931
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 931;

pub fn longest_field_with_last_tie_pipe(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    longest_field_with_last_tie_pipe_impl(text, visit)
}

fn longest_field_with_last_tie_pipe_impl(
    text: &str,
    mut visit: impl FnMut(&str) -> usize,
) -> usize {
    visit({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
    })
}
