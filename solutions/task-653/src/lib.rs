// rust-track: handwritten task-653
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 653;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn first_nonempty_field_hash(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split('#').find(|s| !s.is_empty()).unwrap_or_default()
    })
    .len()
        + record.revision
}
