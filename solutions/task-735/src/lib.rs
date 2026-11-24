// rust-track: handwritten task-735
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 735;

pub fn closing_field_equals(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit('=').next().unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
