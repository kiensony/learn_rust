// rust-track: handwritten task-610
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 610;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn closing_marks_removed_colon(record: &Record) -> usize {
    let text = record.label.as_str();
    (text.trim_end_matches(':')).len() + record.revision
}
