// rust-track: handwritten task-617
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 617;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn closing_field_slash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit('/').next().unwrap_or_default()
    })
    .len()
        + record.revision
}
