// rust-track: handwritten task-1157
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1157;

pub fn prefix_before_final_mark_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    prefix_before_final_mark_colon_impl(text, fallback)
}

fn prefix_before_final_mark_colon_impl<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = text.rsplit_once(':').map_or("", |(a, _)| a);
    if found.is_empty() { fallback } else { found }
}
