// rust-track: handwritten task-749
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 749;

pub fn remainder_after_first_mark_hash(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split_once('#').map_or("", |(_, b)| b)
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
