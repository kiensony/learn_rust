// rust-track: handwritten task-641
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 641;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn remainder_after_first_mark_equals(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split_once('=').map_or("", |(_, b)| b)
    })
    .len()
        + record.revision
}
