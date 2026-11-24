// rust-track: handwritten task-515
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 515;

pub fn first_nonempty_field_colon(text: &str) -> usize {
    (text.split(':').find(|s| !s.is_empty()).unwrap_or_default())
        .chars()
        .count()
}
