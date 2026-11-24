// rust-track: handwritten task-629
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 629;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn closing_marks_removed_pipe(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_end_matches('|')
    })
    .len()
        + record.revision
}
