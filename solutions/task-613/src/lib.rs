// rust-track: handwritten task-613
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 613;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn first_nonempty_field_colon(record: &Record) -> usize {
    let text = record.label.as_str();
    (text.split(':').find(|s| !s.is_empty()).unwrap_or_default()).len() + record.revision
}
