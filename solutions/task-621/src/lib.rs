// rust-track: handwritten task-621
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 621;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn remainder_after_first_mark_slash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split_once('/').map_or("", |(_, b)| b)
    })
    .len()
        + record.revision
}
