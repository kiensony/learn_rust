// rust-track: handwritten task-634
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 634;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn longest_field_with_last_tie_pipe(record: &Record) -> usize {
    longest_field_with_last_tie_pipe_impl(record)
}

fn longest_field_with_last_tie_pipe_impl(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
        + record.revision
}
