// rust-track: handwritten task-743
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 743;

pub fn shortest_field_with_first_tie_equals(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').min_by_key(|s| s.len()).unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
