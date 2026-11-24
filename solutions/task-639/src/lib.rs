// rust-track: handwritten task-639
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 639;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn closing_marks_removed_equals(record: &Record) -> usize {
    closing_marks_removed_equals_impl(record)
}

fn closing_marks_removed_equals_impl(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_end_matches('=')
    })
    .len()
        + record.revision
}
