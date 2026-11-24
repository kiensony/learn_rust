// rust-track: handwritten task-573
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 573;

pub fn prefix_before_final_mark_slash<'a>(text: &'a str) -> &'a str {
    prefix_before_final_mark_slash_impl(text)
}

fn prefix_before_final_mark_slash_impl<'a>(text: &'a str) -> &'a str {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    }
}
