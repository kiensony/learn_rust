// rust-track: handwritten task-374
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 374;

pub fn prefix_before_final_mark_slash(text: &String) -> usize {
    prefix_before_final_mark_slash_impl(text)
}

fn prefix_before_final_mark_slash_impl(text: &String) -> usize {
    {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    }
    .len()
}
