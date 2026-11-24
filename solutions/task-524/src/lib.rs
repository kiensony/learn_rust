// rust-track: handwritten task-524
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 524;

pub fn prefix_before_final_mark_slash(text: &str) -> usize {
    prefix_before_final_mark_slash_impl(text)
}

fn prefix_before_final_mark_slash_impl(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    })
    .chars()
    .count()
}
