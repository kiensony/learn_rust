// rust-track: handwritten task-718
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 718;

pub fn marks_removed_at_both_ends_slash(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_matches('/')
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
