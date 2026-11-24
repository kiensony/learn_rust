// rust-track: handwritten task-364
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 364;

pub fn prefix_before_final_mark_colon(text: &String) -> usize {
    prefix_before_final_mark_colon_impl(text)
}

fn prefix_before_final_mark_colon_impl(text: &String) -> usize {
    text.rsplit_once(':').map_or("", |(a, _)| a).len()
}
