// rust-track: handwritten task-642
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 642;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn prefix_before_final_mark_equals(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit_once('=').map_or("", |(a, _)| a)
    })
    .len()
        + record.revision
}
