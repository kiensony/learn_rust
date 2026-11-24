// rust-track: handwritten task-1179
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1179;

pub fn longest_field_with_last_tie_pipe<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    longest_field_with_last_tie_pipe_impl(text, fallback)
}

fn longest_field_with_last_tie_pipe_impl<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
    };
    if found.is_empty() { fallback } else { found }
}
