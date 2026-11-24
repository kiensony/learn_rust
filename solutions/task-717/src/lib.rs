// rust-track: handwritten task-717
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 717;

pub fn closing_marks_removed_slash(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_end_matches('/')
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
