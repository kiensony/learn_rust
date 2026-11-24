// rust-track: handwritten task-964
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 964;

pub fn leading_marks_removed_slash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.trim_start_matches('/')
            })
            .len()
        })
        .collect()
}
