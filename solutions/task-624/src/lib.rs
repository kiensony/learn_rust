// rust-track: handwritten task-624
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 624;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn longest_field_with_last_tie_slash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
        + record.revision
}
