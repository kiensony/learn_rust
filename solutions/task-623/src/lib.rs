// rust-track: handwritten task-623
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 623;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn first_nonempty_field_slash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').find(|s| !s.is_empty()).unwrap_or_default()
    })
    .len()
        + record.revision
}
