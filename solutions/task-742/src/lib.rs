// rust-track: handwritten task-742
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 742;

pub fn longest_field_with_last_tie_equals(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').max_by_key(|s| s.len()).unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
