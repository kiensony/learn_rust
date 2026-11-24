// rust-track: handwritten task-650
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 650;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn marks_removed_at_both_ends_hash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_matches('#')
    })
    .len()
        + record.revision
}
