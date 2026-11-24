// rust-track: handwritten task-611
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 611;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn marks_removed_at_both_ends_colon(record: &Record) -> usize {
    let text = record.label.as_str();
    (text.trim_matches(':')).len() + record.revision
}
