// rust-track: handwritten task-720
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 720;

pub fn prefix_before_final_mark_slash(record: Option<&str>) -> Option<u16> {
    prefix_before_final_mark_slash_impl(record)
}

fn prefix_before_final_mark_slash_impl(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
