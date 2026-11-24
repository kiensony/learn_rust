// rust-track: handwritten task-705
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 705;

pub fn leading_field_colon(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = text.split(':').next().unwrap_or_default();
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
