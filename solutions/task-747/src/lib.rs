// rust-track: handwritten task-747
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 747;

pub fn closing_marks_removed_hash(record: Option<&str>) -> Option<u16> {
    closing_marks_removed_hash_impl(record)
}

fn closing_marks_removed_hash_impl(record: Option<&str>) -> Option<u16> {
    let text = record?;
    let field = {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_end_matches('#')
    };
    if field.bytes().all(|b| b.is_ascii_digit()) {
        field.parse().ok()
    } else {
        None
    }
}
