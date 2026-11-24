// rust-track: handwritten task-585
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 585;

pub fn longest_field_with_last_tie_pipe<'a>(text: &'a str) -> &'a str {
    longest_field_with_last_tie_pipe_impl(text)
}

fn longest_field_with_last_tie_pipe_impl<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
    }
}
