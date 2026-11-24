// rust-track: handwritten task-630
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 630;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn marks_removed_at_both_ends_pipe(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_matches('|')
    })
    .len()
        + record.revision
}
