// rust-track: handwritten task-745
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 745;

pub fn closing_field_hash(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.rsplit('#').next().unwrap_or_default()
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
