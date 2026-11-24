// rust-track: handwritten task-564
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 564;

pub fn first_nonempty_field_colon<'a>(text: &'a str) -> &'a str {
    text.split(':').find(|s| !s.is_empty()).unwrap_or_default()
}
