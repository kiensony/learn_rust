// rust-track: handwritten task-647
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 647;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn closing_field_hash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.rsplit('#').next().unwrap_or_default()
    })
    .len()
        + record.revision
}
