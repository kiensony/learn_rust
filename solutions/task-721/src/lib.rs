// rust-track: handwritten task-721
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 721;

pub fn first_nonempty_field_slash(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').find(|s| !s.is_empty()).unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
