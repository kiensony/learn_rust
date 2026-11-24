// rust-track: handwritten task-910
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 910;

pub fn first_nonempty_field_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit(text.split(':').find(|s| !s.is_empty()).unwrap_or_default())
}
