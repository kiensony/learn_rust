// rust-track: handwritten task-608
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 608;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn closing_field_colon(record: &Record) -> usize {
    let text = record.label.as_str();
    (text.rsplit(':').next().unwrap_or_default()).len() + record.revision
}
