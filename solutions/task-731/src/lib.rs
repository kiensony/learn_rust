// rust-track: handwritten task-731
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 731;

pub fn first_nonempty_field_pipe(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').find(|s| !s.is_empty()).unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
