// rust-track: handwritten task-612
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 612;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn prefix_before_final_mark_colon(record: &Record) -> usize {
    prefix_before_final_mark_colon_impl(record)
}

fn prefix_before_final_mark_colon_impl(record: &Record) -> usize {
    let text = record.label.as_str();
    (text.rsplit_once(':').map_or("", |(a, _)| a)).len() + record.revision
}
