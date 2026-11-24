// rust-track: handwritten task-971
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 971;

pub fn shortest_field_with_first_tie_slash<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.strip_prefix("path/").unwrap_or(text);
                text.split('/').min_by_key(|s| s.len()).unwrap_or_default()
            })
            .len()
        })
        .collect()
}
