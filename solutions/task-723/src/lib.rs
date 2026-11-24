// rust-track: handwritten task-723
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 723;

pub fn shortest_field_with_first_tie_slash(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').min_by_key(|s| s.len()).unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
