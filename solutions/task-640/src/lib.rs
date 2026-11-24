// rust-track: handwritten task-640
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 640;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn marks_removed_at_both_ends_equals(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_matches('=')
    })
    .len()
        + record.revision
}
