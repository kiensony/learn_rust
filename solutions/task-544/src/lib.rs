// rust-track: handwritten task-544
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 544;

pub fn prefix_before_final_mark_equals(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit_once('=').map_or("", |(a, _)| a)
    })
    .chars()
    .count()
}
