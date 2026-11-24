// rust-track: handwritten task-728
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 728;

pub fn marks_removed_at_both_ends_pipe(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_matches('|')
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
