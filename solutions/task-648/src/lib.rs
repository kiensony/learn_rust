// rust-track: handwritten task-648
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 648;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn leading_marks_removed_hash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_start_matches('#')
    })
    .len()
        + record.revision
}
