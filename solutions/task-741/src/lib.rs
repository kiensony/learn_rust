// rust-track: handwritten task-741
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 741;

pub fn first_nonempty_field_equals(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').find(|s| !s.is_empty()).unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
