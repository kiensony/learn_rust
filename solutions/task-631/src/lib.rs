// rust-track: handwritten task-631
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 631;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn remainder_after_first_mark_pipe(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split_once('|').map_or("", |(_, b)| b)
    })
    .len()
        + record.revision
}
