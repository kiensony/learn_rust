// rust-track: handwritten task-965
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 965;

pub fn closing_marks_removed_slash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.trim_end_matches('/')
            })
            .len()
        })
        .collect()
}
