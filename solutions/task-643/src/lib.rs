// rust-track: handwritten task-643
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 643;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn first_nonempty_field_equals(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').find(|s| !s.is_empty()).unwrap_or_default()
    })
    .len()
        + record.revision
}
