// rust-track: handwritten task-620
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 620;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn marks_removed_at_both_ends_slash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_matches('/')
    })
    .len()
        + record.revision
}
