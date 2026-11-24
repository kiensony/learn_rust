// rust-track: handwritten task-725
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 725;

pub fn closing_field_pipe(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.rsplit('|').next().unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
