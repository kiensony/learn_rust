// rust-track: handwritten task-712
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 712;

pub fn longest_field_with_last_tie_colon(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = text.split(':').max_by_key(|s| s.len()).unwrap_or_default();
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
