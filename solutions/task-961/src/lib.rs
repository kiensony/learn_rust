// rust-track: handwritten task-961
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 961;

pub fn shortest_field_with_first_tie_colon<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| (text.split(':').min_by_key(|s| s.len()).unwrap_or_default()).len())
        .collect()
}
