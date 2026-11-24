// rust-track: handwritten task-919
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 919;

pub fn prefix_before_final_mark_slash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    prefix_before_final_mark_slash_impl(text, visit)
}

fn prefix_before_final_mark_slash_impl(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    })
}
