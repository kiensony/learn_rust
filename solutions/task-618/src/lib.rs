// rust-track: handwritten task-618
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 618;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn leading_marks_removed_slash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_start_matches('/')
    })
    .len()
        + record.revision
}
