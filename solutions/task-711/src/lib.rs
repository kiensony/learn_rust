// rust-track: handwritten task-711
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 711;

pub fn first_nonempty_field_colon(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = text.split(':').find(|s| !s.is_empty()).unwrap_or_default();
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
