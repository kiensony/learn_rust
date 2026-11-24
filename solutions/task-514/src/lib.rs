// rust-track: handwritten task-514
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 514;

pub fn prefix_before_final_mark_colon(text: &str) -> usize {
    prefix_before_final_mark_colon_impl(text)
}

fn prefix_before_final_mark_colon_impl(text: &str) -> usize {
    (text.rsplit_once(':').map_or("", |(a, _)| a))
        .chars()
        .count()
}
