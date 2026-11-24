// rust-track: handwritten task-459
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 459;

pub fn closing_field_colon(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    (text.rsplit(':').next().unwrap_or_default()).len()
}
