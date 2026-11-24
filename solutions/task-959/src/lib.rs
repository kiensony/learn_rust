// rust-track: handwritten task-959
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 959;

pub fn first_nonempty_field_colon<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    records
        .into_iter()
        .map(|text| (text.split(':').find(|s| !s.is_empty()).unwrap_or_default()).len())
        .collect()
}
