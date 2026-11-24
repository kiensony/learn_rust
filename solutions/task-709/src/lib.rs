// rust-track: handwritten task-709
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 709;

pub fn marks_removed_at_both_ends_colon(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = text.trim_matches(':');
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
