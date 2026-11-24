// rust-track: handwritten task-625
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 625;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn shortest_field_with_first_tie_slash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split('/').min_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
        + record.revision
}
