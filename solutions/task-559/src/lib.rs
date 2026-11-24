// rust-track: handwritten task-559
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 559;

pub fn closing_field_colon<'a>(text: &'a str) -> &'a str {
    text.rsplit(':').next().unwrap_or_default()
}
