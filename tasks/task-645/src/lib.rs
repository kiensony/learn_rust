#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn shortest_field_with_first_tie_equals(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=')
            .min_by_key(|s| s.chars().count())
            .unwrap_or_default()
    })
    .chars()
    .count()
        + record.revision
}
