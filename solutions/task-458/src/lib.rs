// rust-track: handwritten task-458
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 458;

pub fn leading_field_colon(fields: (&str, &str)) -> usize {
    let text = if fields.0.len() >= fields.1.len() {
        fields.0
    } else {
        fields.1
    };
    (text.split(':').next().unwrap_or_default()).len()
}
