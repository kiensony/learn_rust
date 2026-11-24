// rust-track: handwritten task-638
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 638;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn leading_marks_removed_equals(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_start_matches('=')
    })
    .len()
        + record.revision
}
