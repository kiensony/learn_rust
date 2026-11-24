// rust-track: handwritten task-649
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 649;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn closing_marks_removed_hash(record: &Record) -> usize {
    closing_marks_removed_hash_impl(record)
}

fn closing_marks_removed_hash_impl(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_end_matches('#')
    })
    .len()
        + record.revision
}
