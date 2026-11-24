// rust-track: handwritten task-991
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 991;

pub fn shortest_field_with_first_tie_equals<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| {
            ({
                let text: &str = &text;
                let text = text.split_once(';').map_or(text, |(data, _)| data);
                text.split('=').min_by_key(|s| s.len()).unwrap_or_default()
            })
            .len()
        })
        .collect()
}
