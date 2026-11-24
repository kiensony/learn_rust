// rust-track: handwritten task-627
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 627;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn closing_field_pipe(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.rsplit('|').next().unwrap_or_default()
    })
    .len()
        + record.revision
}
