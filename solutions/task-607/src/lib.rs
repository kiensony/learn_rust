// rust-track: handwritten task-607
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 607;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn leading_field_colon(record: &Record) -> usize {
    let text = record.label.as_str();
    (text.split(':').next().unwrap_or_default()).len() + record.revision
}
