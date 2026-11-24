// rust-track: handwritten task-724
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 724;

pub fn leading_field_pipe(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').next().unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
