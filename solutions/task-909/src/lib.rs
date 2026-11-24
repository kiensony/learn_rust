// rust-track: handwritten task-909
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 909;

pub fn prefix_before_final_mark_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    prefix_before_final_mark_colon_impl(text, visit)
}

fn prefix_before_final_mark_colon_impl(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit(text.rsplit_once(':').map_or("", |(a, _)| a))
}
