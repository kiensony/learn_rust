// rust-track: handwritten task-719
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 719;

pub fn remainder_after_first_mark_slash(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split_once('/').map_or("", |(_, b)| b)
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
