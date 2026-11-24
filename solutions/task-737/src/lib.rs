// rust-track: handwritten task-737
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 737;

pub fn closing_marks_removed_equals(record: Option<&str>) -> Option<u16> {
    closing_marks_removed_equals_impl(record)
}

fn closing_marks_removed_equals_impl(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_end_matches('=')
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
