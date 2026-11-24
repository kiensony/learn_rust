// rust-track: handwritten task-1167
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1167;

pub fn prefix_before_final_mark_slash<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    prefix_before_final_mark_slash_impl(text, fallback)
}

fn prefix_before_final_mark_slash_impl<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    };
    if found.is_empty() { fallback } else { found }
}
