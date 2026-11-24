// rust-track: handwritten task-708
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 708;

pub fn closing_marks_removed_colon(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = text.trim_end_matches(':');
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
