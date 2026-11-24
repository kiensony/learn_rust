// rust-track: handwritten task-563
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 563;

pub fn prefix_before_final_mark_colon<'a>(text: &'a str) -> &'a str {
    prefix_before_final_mark_colon_impl(text)
}

fn prefix_before_final_mark_colon_impl<'a>(text: &'a str) -> &'a str {
    text.rsplit_once(':').map_or("", |(a, _)| a)
}
