// rust-track: handwritten task-622
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 622;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn prefix_before_final_mark_slash(record: &Record) -> usize {
    prefix_before_final_mark_slash_impl(record)
}

fn prefix_before_final_mark_slash_impl(record: &Record) -> usize {
    let text = record.label.as_str();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit_once('/').map_or("", |(a, _)| a)
    })
    .len()
        + record.revision
}
