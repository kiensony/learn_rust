// rust-track: handwritten task-558
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 558;

pub fn leading_field_colon<'a>(text: &'a str) -> &'a str {
    text.split(':').next().unwrap_or_default()
}
