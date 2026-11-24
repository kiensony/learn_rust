// rust-track: handwritten task-960
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 960;

pub fn longest_field_with_last_tie_colon<'a>(
    records: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| (text.split(':').max_by_key(|s| s.len()).unwrap_or_default()).len())
        .collect()
}
