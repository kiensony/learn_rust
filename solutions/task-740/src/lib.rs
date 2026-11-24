// rust-track: handwritten task-740
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 740;

pub fn prefix_before_final_mark_equals(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit_once('=').map_or("", |(a, _)| a)
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
