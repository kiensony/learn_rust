// rust-track: handwritten task-970
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 970;

pub fn longest_field_with_last_tie_slash<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.split('/').max_by_key(|s| s.len()).unwrap_or_default()
            })
            .len()
        })
        .collect()
}
